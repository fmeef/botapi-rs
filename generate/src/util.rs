use crate::{
    naming::{get_field_name, get_type_name_str},
    ARRAY_OF, INPUT_FILE, MULTITYPE_ENUM_PREFIX,
};
use std::collections::HashSet;

use crate::schema::{Field, Spec, Type};
use anyhow::Result;
use quote::{format_ident, quote, ToTokens, __private::TokenStream};

/// CycleChecker checks a specific type to avoid member "loops" that confuse rustc. For rustc
/// recursive types have infinite size and trigger a compiler error. We can fix this by running
/// cycle detection on members and breaking any cycles using a Box<T>
struct CycleChecker<'a> {
    spec: &'a Spec,
    visited: HashSet<&'a str>,
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
    F: FnMut(&Field) -> R,
    F: 'a,
{
    t.fields.iter().flat_map(|v| v.iter()).map(func)
}

/// Helper method to walk map a function onto a type's fields
pub(crate) fn field_iter_str<'a, F, R>(t: &'a Type, func: F) -> impl Iterator<Item = R> + 'a
where
    F: FnMut(String) -> R + 'a,
{
    t.fields
        .iter()
        .flat_map(|v| v.iter().map(|f| get_field_name(f)))
        .map(func)
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
    let types = types.map(|v| format_ident!("{}", v.as_ref()));
    let name = format_ident!("{}", name.as_ref());

    quote! {
        impl fmt::Display for #name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let v = match self {
                   #(
                       #name::#types(thing) => thing.to_string()
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
    let t = t.as_ref();
    &t[ARRAY_OF.len() * nested..]
}

/// Hacky workaround to break our dependency on multitype enums for now while serde_urlencoded
/// fixes outstanding issues with untagged enums
pub(crate) fn is_chatid(types: &[String]) -> bool {
    types.len() == 2
        && types.contains(&"String".to_owned())
        && types.contains(&"Integer".to_owned())
}
/// Generate the type for a specific field, depending if we have an array type,
/// a api type that needs to be mapped to a native type, or a choice of types that
/// should be either narrowed down to owe or turned into an enum type
pub(crate) fn choose_type(spec: &Spec, field: &Field, parent: &Type) -> Result<TokenStream> {
    let mytype = &field.types[0];
    let nested = is_array(&mytype);
    let mut checker = CycleChecker::new(spec);
    let t = if nested > 0 {
        let fm = if is_inputfile(field) || (parent.is_media() && field.name == "media") {
            INPUT_FILE.to_owned()
        } else {
            if is_chatid(field.types.as_slice()) {
                "i64".to_owned()
            } else if field.types.len() > 1 {
                get_multitype_name(&field)
            } else {
                mytype[ARRAY_OF.len() * nested..].to_owned()
            }
        };
        let res = type_mapper(&fm);
        let res = format_ident!("{}", res);

        let res = if parent.is_media() && field.name == "media" {
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
        if checker.check_parent(parent, &fm) && !(parent.is_media() && field.name == "media") {
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
        let mytype = if is_inputfile(field) || (parent.is_media() && field.name == "media") {
            INPUT_FILE.to_owned()
        } else {
            if is_chatid(field.types.as_slice()) {
                "i64".to_owned()
            } else if field.types.len() > 1 {
                get_multitype_name(field)
            } else {
                type_mapper(mytype)
            }
        };
        let res = format_ident!("{}", mytype);
        let res = if parent.is_media() && field.name == "media" {
            quote! { Option<#res> }
        } else {
            res.to_token_stream()
        };
        if checker.check_parent(parent, &mytype) && !(parent.is_media() && field.name == "media") {
            quote! {
                Box<#res>
            }
        } else {
            quote!(#res)
        }
    };
    Ok(is_optional(field, t))
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

/// Check if a field should be serialized as json. If false, use a native type
pub(crate) fn is_json(field: &Field) -> bool {
    for t in &field.types {
        for compare in ["Integer", "Boolean", "Float"] {
            if t.ends_with(compare) && !t.starts_with("Array of") {
                return false;
            }
        }
    }
    true
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

impl<'a> CycleChecker<'a> {
    fn new(spec: &'a Spec) -> Self {
        CycleChecker {
            spec,
            visited: HashSet::new(),
        }
    }

    /// Check a type's field for dependency loops
    fn check_parent<T>(&mut self, parent: &'a Type, name: &T) -> bool
    where
        T: AsRef<str>,
    {
        if self.visited.insert(&parent.name) {
            if let Some(field) = &parent.fields {
                for supertype in field {
                    if let Some(supertype) = self.spec.get_type(&supertype.types[0]) {
                        if self.check_parent(supertype, name) {
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
