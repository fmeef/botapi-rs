use std::collections::HashSet;

use anyhow::{anyhow, Result};
use quote::{format_ident, quote, ToTokens, __private::TokenStream};
use schema::{Field, Spec, Type};

pub(crate) mod schema;

pub struct GenerateTypes(Spec);
struct CycleChecker<'a> {
    spec: &'a Spec,
    visited: HashSet<&'a str>,
}

//static MULTITYPE_ENUM_PREFIX: &str = "E";
static ARRAY_OF: &str = "Array of ";
static MEMBER_PREFIX: &str = "tg_";

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
            if parent.name == name.as_ref() {
                return true;
            } else {
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
        }

        parent.name == name.as_ref()
    }
}

impl GenerateTypes {
    pub fn new<T: AsRef<str>>(json: T) -> Result<GenerateTypes> {
        Ok(GenerateTypes(serde_json::from_str(json.as_ref())?))
    }

    pub fn generate_types(&self) -> Result<String> {
        Ok(self.preamble()?.into_token_stream().to_string())
    }

    fn preamble(&self) -> Result<impl ToTokens> {
        let structs = self
            .0
            .types
            .values()
            .filter_map(|v| self.generate_struct(&v.name).ok());
        let e = self.generate_enum(self.0.types.values(), &"GlobalTypes")?;
        let uses = self.generate_use()?;
        let res = quote! {
            #uses
            #( #structs )*
            #e
        };
        Ok(res)
    }

    fn generate_use(&self) -> Result<impl ToTokens> {
        Ok(quote! {
            use serde::{Deserialize, Serialize};
        })
    }

    fn type_mapper<T>(&self, field: &T) -> String
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

    fn generate_enum<'a, T, N>(&'a self, types: T, name: &N) -> Result<impl ToTokens>
    where
        T: Iterator<Item = &'a Type>,
        N: AsRef<str>,
    {
        let types = types.map(|v| format_ident!("{}", v.name));
        let name = format_ident!("{}", name.as_ref());
        let e = quote! {
            pub enum #name {
                #(
                    #types(#types)
                ),*
            }
        };
        Ok(e)
    }

    /// TODO: generate the e_* enum type
    fn choose_type(&self, field: &Field, parent: Option<&Type>) -> Result<TokenStream> {
        let mytype = &field.types[0];
        let nested = self.is_array(&mytype);
        let mut checker = CycleChecker::new(&self.0);
        let t = if nested > 0 {
            let fm = &mytype[ARRAY_OF.len() * nested..];
            let res = self.type_mapper(&fm);
            let res = format_ident!("{}", res);
            let mut quote = quote!();
            for _ in 0..nested {
                let vec = quote! {
                    Vec<
                };
                quote.extend(vec);
            }
            match parent {
                Some(parent) => {
                    if checker.check_parent(parent, &fm) {
                        quote.extend(quote! {
                            Box<#res>
                        });
                    } else {
                        quote.extend(quote! {
                            #res
                        });
                    }
                }
                None => {
                    quote.extend(quote! {
                        #res
                    });
                }
            }
            for _ in 0..nested {
                let vec = quote! {
                    >
                };
                quote.extend(vec);
            }
            quote
        } else {
            let mytype = self.type_mapper(&mytype);
            let t = format_ident!("{}", mytype);
            if let Some(parent) = parent {
                if checker.check_parent(parent, &mytype) {
                    quote! {
                        Box<#t>
                    }
                } else {
                    quote!(#t)
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

    fn generate_struct<T>(&self, name: &T) -> Result<impl ToTokens>
    where
        T: AsRef<str>,
    {
        let t = self
            .0
            .get_type(name)
            .ok_or_else(|| anyhow!("type not found"))?;
        let typename = format_ident!("{}", t.name);
        let def = &Vec::<Field>::new();
        let fields = t.fields.as_ref().unwrap_or(&def);
        let fieldnames = fields
            .iter()
            .map(|v| &v.name)
            .map(|v| format_ident!("{}{}", MEMBER_PREFIX, v));
        let serdenames = fields.iter().map(|v| &v.name);
        let fieldtypes = fields
            .iter()
            .filter_map(|v| self.choose_type(&v, Some(&t)).ok());
        let res = quote! {
            #[derive(Serialize, Deserialize)]
            pub struct #typename {
                #(
                    #[serde(rename = #serdenames)]
                    pub #fieldnames: #fieldtypes
                ),*
            }
        };
        Ok(res)
    }
}
