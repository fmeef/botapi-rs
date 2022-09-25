use crate::schema::{Field, Spec, Type};
use crate::{
    naming::{get_field_name, get_type_name_str},
    ARRAY_OF, INPUT_FILE, MULTITYPE_ENUM_PREFIX,
};
use anyhow::Result;
use quote::{format_ident, quote, ToTokens, __private::TokenStream};
use std::collections::HashSet;
use std::sync::Arc;

pub(crate) trait ChooserFn {
    fn cb<'b, 'c>(self: &Self, types: &TypeChooserOpts<'b, 'c>) -> String;
}

impl<F> ChooserFn for F
where
    F: for<'a, 'b, 'c> Fn(&'a TypeChooserOpts<'b, 'c>) -> String,
{
    fn cb<'b, 'c>(self: &Self, types: &TypeChooserOpts<'b, 'c>) -> String {
        self(types)
    }
}

/// CycleChecker checks a specific type to avoid member "loops" that confuse rustc. For rustc
/// recursive types have infinite size and trigger a compiler error. We can fix this by running
/// cycle detection on members and breaking any cycles using a Box<T>
pub(crate) struct CycleChecker {
    spec: Arc<Spec>,
    visited: HashSet<String>,
}

/// Check if a field is an "InputFile" for special treatment
pub(crate) fn is_inputfile(field: &Field) -> bool {
    is_inputfile_types(field.types.as_slice())
}

/// Check if a field is an "InputFile" for special treatment
pub(crate) fn is_inputfile_types(field: &[String]) -> bool {
    field.contains(&INPUT_FILE.to_owned())
}

/// Helper method to walk map a function onto a type's fields
pub(crate) fn field_iter<'a, F, R>(t: &'a Type, func: F) -> impl Iterator<Item = R> + 'a
where
    F: FnMut(&'a Field) -> R,
    F: 'a,
    R: 'a,
{
    t.fields.iter().flat_map(|v| v.iter()).map(func)
}

/// Helper method to walk map a function onto a type's fields
pub(crate) fn field_iter_str<'a, F, R>(t: &'a Type, func: F) -> impl Iterator<Item = R> + 'a
where
    F: FnMut(String) -> R + 'a,
    R: 'a,
{
    t.fields
        .iter()
        .flat_map(|v| v.iter().map(|f| get_field_name(f)))
        .map(func)
}

pub(crate) fn get_multitype_name_types<T>(name: &T, types: &[String]) -> String
where
    T: AsRef<str>,
{
    if is_inputfile_types(types) {
        INPUT_FILE.to_owned()
    } else {
        let fieldname = get_type_name_str(name);
        format!("{}{}", MULTITYPE_ENUM_PREFIX, fieldname)
    }
}

/// Get the name for a multitype enum
pub(crate) fn get_multitype_name(fieldname: &Field) -> String {
    if is_inputfile(fieldname) {
        INPUT_FILE.to_owned()
    } else {
        let fieldname = &fieldname.name;
        let fieldname = get_type_name_str(&fieldname);
        format!("{}{}", MULTITYPE_ENUM_PREFIX, fieldname)
    }
}

/// Check if a json spec type name is an "array" and return the offset of the actual type name
pub(crate) fn is_array<T>(name: &T) -> usize
where
    T: AsRef<str>,
{
    name.as_ref().matches(ARRAY_OF).count()
}

/// Helper function to generate a std::fmt::Display implementation for multiple types
pub(crate) fn generate_fmt_display_enum<'a, T, V, U>(name: &T, types: V) -> TokenStream
where
    T: AsRef<str>,
    V: Iterator<Item = U>,
    U: AsRef<str> + 'a,
{
    let name = format_ident!("{}", name.as_ref());
    let types = types.map(|v| {
        let t = format_ident!("{}", v.as_ref());
        if is_primative(v) {
            quote! {  #name::#t(thing) => thing.to_string() }
        } else {
            quote! { #name::#t(thing) => serde_json::to_string(thing).unwrap_or_else(|err| format!("invalid: {err}") ) }
        }
    });

    quote! {
        impl fmt::Display for #name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let v = match self {
                   #(
                       #types
                   ),*
                };
                write!(f, "{}", v)?;
                Ok(())
            }
        }
    }
}

/// Take a type name and return a slice without a leading "Array of.*"
pub(crate) fn type_without_array<'a, T>(t: &'a T) -> &'a str
where
    T: AsRef<str>,
{
    let nested = is_array(t);
    if nested > 0 {
        let t = t.as_ref();
        &t[ARRAY_OF.len() * nested..]
    } else {
        t.as_ref()
    }
}

/// Hacky workaround to break our dependency on multitype enums for now while serde_urlencoded
/// fixes outstanding issues with untagged enums
pub(crate) fn is_chatid(types: &[String]) -> bool {
    types.len() == 2
        && types.contains(&"String".to_owned())
        && types.contains(&"Integer".to_owned())
}

pub(crate) struct TypeChooserOpts<'a, 'b> {
    pub(crate) types: &'a [String],
    pub(crate) is_media: bool,
    pub(crate) nested: usize,
    pub(crate) name: &'b str,
}

pub(crate) struct ChooseType<'a> {
    spec: Arc<Spec>,
    type_chooser: Box<dyn ChooserFn + 'a>,
}

impl<'a> ChooseType<'a> {
    pub(crate) fn new<U>(spec: Arc<Spec>, type_chooser: U) -> Self
    where
        U: for<'d, 'b, 'c> Fn(&'d TypeChooserOpts<'b, 'c>) -> String + 'a,
        U: 'a,
    {
        let type_chooser = Box::new(type_chooser);
        Self { spec, type_chooser }
    }

    /// Generate the type for a specific field, depending if we have an array type,
    /// a api type that needs to be mapped to a native type, or a choice of types that
    /// should be either narrowed down to owe or turned into an enum type
    pub(crate) fn choose_type<T>(
        &self,
        types: &[String],
        parent: Option<&Type>,
        name: &T,
        optional: bool,
    ) -> Result<TokenStream>
    where
        T: AsRef<str>,
    {
        let is_media = parent.map(|t| t.is_media()).unwrap_or(false);
        let mut checker = CycleChecker::new(Arc::clone(&self.spec));
        let nested = is_array(&types[0]);
        let opts = TypeChooserOpts {
            types,
            is_media,
            nested,
            name: name.as_ref(),
        };

        let mytype = self.type_chooser.cb(&opts);
        let mytype = type_mapper(&mytype);
        let nested = is_array(&mytype);
        let t = if nested > 0 {
            let mytype = type_without_array(&mytype);
            let res = type_mapper(&mytype);
            let res = format_ident!("{}", res);

            let res = if is_media && name.as_ref() == "media" {
                quote! { Option<#res> }
            } else {
                res.to_token_stream()
            };
            let mut quote = quote!();
            for _ in 0..nested {
                let vec = quote! {
                    Vec<
                };
                quote.extend(vec);
            }
            let checked = if let Some(parent) = parent {
                checker.check_parent(parent, &mytype)
            } else {
                false
            };

            if checked && !(is_media && name.as_ref() == "media") {
                quote.extend(quote! {
                    Box<#res>
                });
            } else {
                quote.extend(quote! {
                    #res
                });
            }
            for _ in 0..nested {
                let vec = quote! {
                    >
                };
                quote.extend(vec);
            }
            quote
        } else {
            let res = format_ident!("{}", mytype);
            let res = if is_media && name.as_ref() == "media" {
                quote! { Option<#res> }
            } else {
                res.to_token_stream()
            };
            let checked = if let Some(parent) = parent {
                checker.check_parent(parent, &mytype)
            } else {
                false
            };
            if checked && !(is_media && name.as_ref() == "media") {
                quote! {
                    Box<#res>
                }
            } else {
                quote!(#res)
            }
        };
        Ok(if optional {
            quote! {
                Option<#t>
            }
        } else {
            t.to_token_stream()
        })
    }
}

/// Conditionally generate an Option<T> out of a field
pub(crate) fn is_optional<T>(field: &Field, tokenstram: T) -> TokenStream
where
    T: ToTokens,
{
    if !field.required {
        quote! {
            Option<#tokenstram>
        }
    } else {
        tokenstram.to_token_stream()
    }
}

pub(crate) fn is_json_types(types: &[String]) -> bool {
    for t in types {
        for compare in ["Integer", "Boolean", "Float"] {
            if t.ends_with(compare) && !t.starts_with("Array of") {
                return false;
            }
        }
    }
    true
}

/// Check if a field should be serialized as json. If false, use a native type
pub(crate) fn is_json(field: &Field) -> bool {
    is_json_types(&field.types)
}

/// Returns true if a REST type is primative (does not map to a serde type)
pub(crate) fn is_primative<T>(field: T) -> bool
where
    T: AsRef<str>,
{
    match field.as_ref() {
        "Integer" => true,
        "Boolean" => true,
        "Float" => true,
        _ => false,
    }
}

/// Map api spec REST types onto native rust types
pub(crate) fn type_mapper<T>(field: &T) -> String
where
    T: AsRef<str>,
{
    match field.as_ref() {
        "Integer" => "i64".to_owned(),
        "Boolean" => "bool".to_owned(),
        "Float" => "f64".to_owned(),
        _ => field.as_ref().to_owned(),
    }
}

/// Map api spec REST types onto native rust types
#[allow(dead_code)]
pub(crate) fn type_mapper_v2<T>(field: &T) -> &str
where
    T: AsRef<str>,
{
    match field.as_ref() {
        "Integer" => "i64",
        "Boolean" => "bool",
        "Float" => "f64",
        x => x,
    }
}

impl CycleChecker {
    pub(crate) fn new(spec: Arc<Spec>) -> Self {
        CycleChecker {
            spec,
            visited: HashSet::new(),
        }
    }

    pub(crate) fn check_parent(&mut self, parent: &Type, name: &str) -> bool {
        if self.spec.is_boxed(name) {
            false
        } else {
            if !self.check_parent_i(parent, name) {
                self.spec.box_type(name.to_owned());
                false
            } else {
                true
            }
        }
    }

    /// Check a type's field for dependency loops
    fn check_parent_i(&mut self, parent: &Type, name: &str) -> bool {
        if self.visited.insert(parent.name.clone()) {
            if let Some(field) = &parent.fields {
                for supertype in field {
                    if let Some(supertype) = self.spec.clone().get_type(&supertype.types[0]) {
                        if self.check_parent_i(supertype, &name) {
                            return true;
                        }
                    }
                }
            }
        }

        parent.name == name.as_ref()
    }
}

/// Helper trait for turing &str things into doc comments
pub(crate) trait IntoComment {
    fn into_comment(&self) -> TokenStream;
}

impl<T> IntoComment for T
where
    T: AsRef<str>,
{
    fn into_comment(&self) -> TokenStream {
        let comment = self.as_ref();
        quote! {
            #[doc = #comment ]
        }
    }
}
