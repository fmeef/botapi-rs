use std::collections::HashSet;

use anyhow::{anyhow, Result};
use quote::{format_ident, quote, ToTokens, __private::TokenStream};
use schema::{Field, Spec, Type};

pub(crate) mod schema;

pub struct Generate(Spec);

pub struct GenerateTypes<'a>(&'a Spec);
pub struct GenerateMethods<'a>(&'a Spec);

struct CycleChecker<'a> {
    spec: &'a Spec,
    visited: HashSet<&'a str>,
}

pub static TELEGRAM_API: &str = "https://api.telegram.org";

static MULTITYPE_ENUM_PREFIX: &str = "E";
static ARRAY_OF: &str = "Array of ";
static MEMBER_PREFIX: &str = "tg_";

macro_rules! field_iter_str {
    ($ty:expr, $func:expr) => {{
        let res = $ty
            .fields
            .iter()
            .flat_map(|v| v.iter().map(|f| f.name.as_str()))
            .map($func);

        res
    }};
}

macro_rules! field_iter {
    ($ty:expr, $func:expr) => {{
        let res = $ty.fields.iter().flat_map(|v| v.iter()).map($func);

        res
    }};
}

fn type_mapper<T>(field: &T) -> String
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

impl<'a> GenerateTypes<'a> {
    pub fn generate_types(&self) -> Result<String> {
        Ok(self.preamble()?.into_token_stream().to_string())
    }

    fn preamble(&self) -> Result<TokenStream> {
        let structs = self
            .0
            .types
            .values()
            .filter_map(|v| self.generate_struct(&v.name).ok());

        let traits = self
            .0
            .types
            .values()
            .filter_map(|v| self.generate_trait(&v).ok());
        let impls = self
            .0
            .types
            .values()
            .filter_map(|v| self.generate_impl(&v.name).ok());
        let typeenums = self.generate_multitype_enums()?;
        let enums = self.generate_enum(self.0.types.values(), &"GlobalTypes")?;
        let uses = self.generate_use()?;
        let res = quote! {
            #uses
            #( #traits )*
            #( #structs )*
            #( #impls )*
            #enums
            #typeenums
        };
        Ok(res)
    }

    fn generate_use(&self) -> Result<TokenStream> {
        Ok(quote! {
            use serde::{Deserialize, Serialize};
        })
    }

    fn get_multitype_name<T>(&self, typename: &T, fieldname: &T) -> String
    where
        T: AsRef<str>,
    {
        format!(
            "{}{}{}",
            MULTITYPE_ENUM_PREFIX,
            typename.as_ref(),
            fieldname.as_ref()
        )
    }

    fn generate_multitype_enums(&self) -> Result<TokenStream> {
        let mut tokens = quote!();

        for jsontype in self.0.types.values() {
            if let Some(fields) = jsontype.fields.as_ref() {
                for field in fields {
                    if field.types.len() > 1 {
                        let name = self.get_multitype_name(&jsontype.name, &field.name);
                        let typeiter: Vec<String> =
                            field.types.iter().map(|t| type_mapper(&t)).collect();
                        let t = self.generate_enum_str(typeiter.as_slice(), &name)?;
                        tokens.extend(t);
                    }
                }
            }
        }

        Ok(tokens)
    }

    fn generate_enum_str<N, I>(&self, types: &[I], name: &N) -> Result<TokenStream>
    where
        N: AsRef<str>,
        I: AsRef<str>,
    {
        let names_iter = types
            .iter()
            .map(|v| v.as_ref()[0..1].to_uppercase() + &v.as_ref()[1..])
            .map(|v| format_ident!("{}", v));
        let types_iter = types.iter().map(|v| format_ident!("{}", v.as_ref()));
        let name = format_ident!("{}", name.as_ref());
        let e = quote! {
            #[derive(Serialize, Deserialize, Debug)]
            pub enum #name {
                #(
                    #names_iter(#types_iter)
                ),*
            }
        };
        Ok(e)
    }

    fn generate_enum<T, N>(&'a self, types: T, name: &N) -> Result<impl ToTokens>
    where
        T: Iterator<Item = &'a Type>,
        N: AsRef<str>,
    {
        let types = types.map(|v| format_ident!("{}", v.name));
        let name = format_ident!("{}", name.as_ref());
        let e = quote! {
            #[derive(Serialize, Deserialize, Debug)]
            pub enum #name {
                #(
                    #types(#types)
                ),*
            }
        };
        Ok(e)
    }

    /// TODO: generate the e_* enum type
    fn choose_type(&self, field: &Field, parent: &Type) -> Result<TokenStream> {
        let mytype = &field.types[0];
        let nested = self.is_array(&mytype);
        let mut checker = CycleChecker::new(&self.0);
        let t = if nested > 0 {
            let fm = if field.types.len() > 1 {
                self.get_multitype_name(&parent.name, &field.name)
            } else {
                mytype[ARRAY_OF.len() * nested..].to_owned()
            };
            let res = type_mapper(&fm);
            let res = format_ident!("{}", res);
            let mut quote = quote!();
            for _ in 0..nested {
                let vec = quote! {
                    Vec<
                };
                quote.extend(vec);
            }
            if checker.check_parent(parent, &fm) {
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
            let mytype = if field.types.len() > 1 {
                self.get_multitype_name(&parent.name, &field.name)
            } else {
                type_mapper(&mytype)
            };
            let t = format_ident!("{}", mytype);
            if checker.check_parent(parent, &mytype) {
                quote! {
                    Box<#t>
                }
            } else {
                quote!(#t)
            }
        };
        Ok(t)
    }

    fn is_array<T>(&self, name: &T) -> usize
    where
        T: AsRef<str>,
    {
        name.as_ref().matches(ARRAY_OF).count()
    }

    fn generate_impl<T>(&self, name: &T) -> Result<TokenStream>
    where
        T: AsRef<str>,
    {
        let t = self
            .0
            .get_type(name)
            .ok_or_else(|| anyhow!("type not found"))?;

        let typename = format_ident!("{}", t.name);
        let fieldnames = field_iter_str!(&t, |v| format_ident!("get_{}{}", MEMBER_PREFIX, v));
        let returnnames = field_iter_str!(&t, |v| format_ident!("{}{}", MEMBER_PREFIX, v));

        let fieldtypes = field_iter!(&t, |v| self.choose_type(v, &t).ok());

        let res = quote! {
            #[allow(dead_code)]
            impl #typename {
                #(
                    pub fn #fieldnames<'a>(&'a self) -> &'a #fieldtypes {
                        &self.#returnnames
                    }
                )*
            }

        };
        Ok(res)
    }

    fn generate_trait(&self, t: &Type) -> Result<TokenStream> {
        let typename = format_ident!("Trait{}", t.name);
        let fieldnames = field_iter_str!(&t, |v| format_ident!("get_{}{}", MEMBER_PREFIX, v));
        let fieldtypes = field_iter!(&t, |v| self.choose_type(v, &t).ok());
        let supertraits = if let Some(subtypes) = t.subtypes.as_ref() {
            let subtypes = subtypes.iter().map(|v| format_ident!("Trait{}", v));
            quote! {
                : #( #subtypes )+*
            }
        } else {
            quote!()
        };
        let res = quote! {
            trait #typename #supertraits {
                #(
                    fn #fieldnames() -> #fieldtypes;
                )*
            }
        };
        Ok(res)
    }

    fn generate_struct<T>(&self, name: &T) -> Result<TokenStream>
    where
        T: AsRef<str>,
    {
        let t = self
            .0
            .get_type(name)
            .ok_or_else(|| anyhow!("type not found"))?;
        let typename = format_ident!("{}", t.name);
        let fieldnames = field_iter_str!(&t, |v| format_ident!("{}{}", MEMBER_PREFIX, v));
        let serdenames = field_iter_str!(&t, |v| v);
        let fieldtypes = field_iter!(&t, |v| self.choose_type(v, &t).ok());
        let res = quote! {
            #[derive(Serialize, Deserialize, Debug)]
            pub struct #typename {
                #(
                    #[serde(rename = #serdenames)]
                    #fieldnames: #fieldtypes
                ),*
            }
        };
        Ok(res)
    }
}

impl Generate {
    pub fn new<T: AsRef<str>>(json: T) -> Result<Generate> {
        Ok(Generate(serde_json::from_str(json.as_ref())?))
    }

    pub fn generate_types(&self) -> Result<String> {
        let generate_types = GenerateTypes(&self.0);
        generate_types.generate_types()
    }
}
