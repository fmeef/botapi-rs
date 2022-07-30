use std::{cell::RefCell, collections::HashSet};

use anyhow::{anyhow, Result};
use quote::{format_ident, quote, ToTokens, __private::TokenStream};
use schema::{Spec, Type};
use util::*;

pub(crate) mod schema;
pub(crate) mod util;

pub struct Generate(Spec);

pub struct GenerateTypes<'a> {
    spec: &'a Spec,
    multitypes: RefCell<HashSet<String>>,
}
pub struct GenerateMethods<'a>(&'a Spec);

pub static TELEGRAM_API: &str = "https://api.telegram.org";

static MULTITYPE_ENUM_PREFIX: &str = "E";
static ARRAY_OF: &str = "Array of ";
static MEMBER_PREFIX: &str = "tg_";
static INPUT_FILE: &str = "InputFile";

impl<'a> GenerateTypes<'a> {
    pub fn generate_types(&self) -> Result<String> {
        Ok(self.preamble()?.into_token_stream().to_string())
    }

    fn preamble(&self) -> Result<TokenStream> {
        let structs = self.spec.types.values().filter_map(|v| {
            if v.fields.as_ref().map(|f| f.len()).unwrap_or(0) > 0 {
                self.generate_struct(&v.name).ok()
            } else {
                if v.name == INPUT_FILE {
                    Some(self.generate_inputfile_enum())
                } else {
                    let vec = Vec::new();
                    let subtypes = v.subtypes.as_ref().unwrap_or(&vec);
                    let subtypes = subtypes.iter().filter_map(|v| self.spec.get_type(v));
                    self.generate_enum(subtypes, &v.name).ok()
                }
            }
        });

        let traits = self
            .spec
            .types
            .values()
            .filter_map(|v| self.generate_trait(&v).ok());
        let impls = self
            .spec
            .types
            .values()
            .filter_map(|v| self.generate_impl(&v.name).ok());
        let typeenums = self.generate_multitype_enums()?;
        let enums = self.generate_enum(self.spec.types.values(), &"GlobalTypes")?;
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

    fn generate_multitype_enums(&self) -> Result<TokenStream> {
        let mut tokens = quote!();

        for jsontype in self.spec.types.values() {
            if let Some(fields) = jsontype.fields.as_ref() {
                for field in fields {
                    if field.types.len() > 1 {
                        if let Some(name) = self.get_multitype_name(&field.name, &field.types) {
                            let typeiter: Vec<String> =
                                field.types.iter().map(|t| type_mapper(&t)).collect();
                            let t = self.generate_enum_str(typeiter.as_slice(), &name)?;
                            tokens.extend(t);
                        }
                    }
                }
            }
        }

        Ok(tokens)
    }

    fn get_multitype_name<T>(&self, field_name: &T, types: &[String]) -> Option<String>
    where
        T: AsRef<str>,
    {
        let mut multitypes = self.multitypes.borrow_mut();
        let key = types.join("");
        if let None = multitypes.get(&key) {
            let name = get_multitype_name(field_name);
            multitypes.insert(key);
            Some(name)
        } else {
            None
        }
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
            #[serde(untagged)]
            pub enum #name {
                #(
                    #names_iter(#types_iter)
                ),*
            }
        };
        Ok(e)
    }

    fn generate_enum<T, N>(&'a self, types: T, name: &N) -> Result<TokenStream>
    where
        T: Iterator<Item = &'a Type>,
        N: AsRef<str>,
    {
        let types = types.map(|v| format_ident!("{}", v.name));
        let name = format_ident!("{}", name.as_ref());
        let e = quote! {
            #[derive(Serialize, Deserialize, Debug)]
            #[serde(untagged)]
            pub enum #name {
                #(
                    #types(#types)
                ),*
            }
        };
        Ok(e)
    }

    fn generate_inputfile_enum(&self) -> TokenStream {
        let input_file = format_ident!("{}", INPUT_FILE);

        quote! {
            #[derive(Serialize, Deserialize, Debug)]
            pub enum #input_file {
                Bytes(Vec<u8>)
            }
        }
    }

    fn generate_trait_impl<T>(&self, traitname: &T) -> Result<TokenStream>
    where
        T: AsRef<str>,
    {
        let supertype = self
            .spec
            .get_type(traitname)
            .ok_or_else(|| anyhow!("type not found"))?;
        let typename = format_ident!("{}", traitname.as_ref());
        let fieldnames =
            field_iter_str(&supertype, |v| format_ident!("get_{}{}", MEMBER_PREFIX, v));
        let returnnames = field_iter_str(&supertype, |v| format_ident!("{}{}", MEMBER_PREFIX, v));
        let trait_name = format_ident!("Trait{}", traitname.as_ref());
        let fieldtypes = field_iter(&supertype, |v| choose_type(&self.spec, v, &supertype).ok());
        let comments = field_iter(&supertype, |v| v.description.into_comment());

        let res = quote! {
             impl #trait_name for #typename {
                    #(
                        #comments
                        fn #fieldnames<'a>(&'a self) -> &'a #fieldtypes {
                            &self.#returnnames
                        }
                    )*
                }
        };

        Ok(res)
    }

    fn generate_impl<T>(&self, name: &T) -> Result<TokenStream>
    where
        T: AsRef<str>,
    {
        let t = self
            .spec
            .get_type(name)
            .ok_or_else(|| anyhow!("type not found"))?;

        let typename = format_ident!("{}", t.name);
        let fieldnames = field_iter_str(&t, |v| format_ident!("get_{}{}", MEMBER_PREFIX, v));
        let returnnames = field_iter_str(&t, |v| format_ident!("{}{}", MEMBER_PREFIX, v));

        let fieldtypes = field_iter(&t, |v| choose_type(&self.spec, v, &t).ok());
        let comments = field_iter(&t, |v| v.description.into_comment());

        let res = if let Some(subtypes) = t.subtypes.as_ref() {
            let impls = subtypes.iter().map(|v| self.generate_trait_impl(&v).ok());
            quote! {
                #( #impls )*
            }
        } else {
            quote! {
                #[allow(dead_code)]
                impl #typename {
                    #(
                        #comments
                        pub fn #fieldnames<'a>(&'a self) -> &'a #fieldtypes {
                            &self.#returnnames
                        }
                    )*
                }

            }
        };
        Ok(res)
    }

    fn generate_trait(&self, t: &Type) -> Result<TokenStream> {
        let typename = format_ident!("Trait{}", t.name);
        let fieldnames = field_iter_str(&t, |v| format_ident!("get_{}{}", MEMBER_PREFIX, v));
        let fieldtypes = field_iter(&t, |v| choose_type(&self.spec, v, &t).ok());

        let comments = field_iter(&t, |v| v.description.into_comment());
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
                    #comments
                    fn #fieldnames<'a>(&'a self) -> &'a #fieldtypes;
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
            .spec
            .get_type(name)
            .ok_or_else(|| anyhow!("type not found"))?;
        let typename = format_ident!("{}", t.name);
        let fieldnames = field_iter_str(&t, |v| format_ident!("{}{}", MEMBER_PREFIX, v));
        let serdenames = field_iter_str(&t, |v| v);
        let fieldtypes = field_iter(&t, |v| choose_type(&self.spec, v, &t).ok());
        let comments = field_iter(&t, |v| v.description.into_comment());
        let struct_comment = t.description.concat().into_comment();
        let res = quote! {
            #struct_comment
            #[derive(Serialize, Deserialize, Debug)]
            pub struct #typename {
                #(
                    #comments
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
        let generate_types = GenerateTypes {
            spec: &self.0,
            multitypes: RefCell::new(HashSet::new()),
        };
        generate_types.generate_types()
    }

    pub fn generate_methods(&self) -> Result<String> {
        let generate_methods = GenerateMethods(&self.0);
        generate_methods.generate_methods()
    }
}

impl<'a> GenerateMethods<'a> {
    fn generate_methods(&self) -> Result<String> {
        Ok(self.preamble()?.into_token_stream().to_string())
    }

    fn preamble(&self) -> Result<TokenStream> {
        Ok(quote!())
    }
}

trait IntoComment {
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
