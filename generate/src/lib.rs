use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use anyhow::{anyhow, Result};
use quote::{format_ident, quote, ToTokens, __private::TokenStream};
use schema::{Method, Spec, Type};
use util::*;

pub(crate) mod schema;
pub(crate) mod util;

pub struct Generate {
    spec: Spec,
    multitypes: Arc<RwLock<HashMap<String, String>>>,
}

pub struct GenerateTypes<'a> {
    spec: &'a Spec,
    multitypes: Arc<RwLock<HashMap<String, String>>>,
}
pub struct GenerateMethods<'a> {
    spec: &'a Spec,
    multitypes: Arc<RwLock<HashMap<String, String>>>,
}

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
                    let subtypes = subtypes
                        .iter()
                        .filter_map(|v| self.spec.get_type(v))
                        .map(|t| t.name.as_str())
                        .collect::<Vec<&str>>();
                    self.generate_enum_str(subtypes.as_slice(), &v.name).ok()
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
        let enums = self.generate_enum_str(
            self.spec
                .types
                .values()
                .map(|v| v.name.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
            &"GlobalTypes",
        )?;
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
            use std::fmt;
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

                            if !is_json(&field) {
                                let typeiter = field
                                    .types
                                    .iter()
                                    .map(|t| type_mapper(&t))
                                    .map(|t| name_to_uppercase(&t));
                                let t = generate_fmt_display_enum(&name, typeiter);
                                tokens.extend(t);
                            }
                        }
                    }
                }
            }
        }

        for method in self.spec.methods.values() {
            if let Some(fields) = method.fields.as_ref() {
                for field in fields {
                    if field.types.len() > 1 {
                        if let Some(name) = self.get_multitype_name(&field.name, &field.types) {
                            let typeiter: Vec<String> =
                                field.types.iter().map(|t| type_mapper(&t)).collect();
                            let t = self.generate_enum_str(typeiter.as_slice(), &name)?;
                            tokens.extend(t);

                            if !is_json(&field) {
                                let typeiter = field
                                    .types
                                    .iter()
                                    .map(|t| type_mapper(&t))
                                    .map(|t| name_to_uppercase(&t));
                                let t = generate_fmt_display_enum(&name, typeiter);
                                tokens.extend(t);
                            }
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
        let mut multitypes = self
            .multitypes
            .write()
            .expect("failed to lock write access");
        let key = types
            .iter()
            .map(|t| type_without_array(t))
            .collect::<Vec<&str>>()
            .join("");
        if let None = multitypes.get(&key) {
            let name = get_multitype_name(field_name);
            multitypes.insert(key, name.clone());
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
            .map(|v| type_without_array(v))
            .map(|v| name_to_uppercase(&v))
            .map(|v| format_ident!("{}", v));
        let types_iter = types
            .iter()
            .map(|v| type_without_array(v))
            .map(|v| format_ident!("{}", v));
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
        Ok(Generate {
            spec: serde_json::from_str(json.as_ref())?,
            multitypes: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub fn generate_types(&self) -> Result<String> {
        let generate_types = GenerateTypes {
            spec: &self.spec,
            multitypes: Arc::clone(&self.multitypes),
        };
        generate_types.generate_types()
    }

    pub fn generate_methods(&self) -> Result<String> {
        let generate_methods = GenerateMethods {
            spec: &self.spec,
            multitypes: Arc::clone(&self.multitypes),
        };
        generate_methods.generate_methods()
    }
}

impl<'a> GenerateMethods<'a> {
    fn generate_methods(&self) -> Result<String> {
        Ok(self.preamble()?.into_token_stream().to_string())
    }

    fn generate_urlencoding_struct(&self, method: &Method) -> Result<TokenStream> {
        let structname = format_ident!("{}_opts", method.name);
        let res = if let Some(fields) = &method.fields {
            let typenames = fields
                .iter()
                .map(|f| format_ident!("tg_{}", f.name.as_str()));
            let types = fields
                .iter()
                .filter_map(|f| self.choose_type(&f.types, !f.required).ok());
            quote! {
                #[derive(Serialize, Deserialize, Debug)]
                struct #structname {
                    #(
                        #typenames : #types
                    ),*
                }
            }
        } else {
            quote! {
                #[derive(Serialize, Deserialize, Debug)]
                struct #structname();
            }
        };

        Ok(res)
    }

    fn instantiate_urlencoding_struct(&self, method: &Method) -> Result<TokenStream> {
        let structname = format_ident!("{}_opts", method.name);
        let res = if let Some(fields) = &method.fields {
            let typenames = fields
                .iter()
                .map(|f| format_ident!("tg_{}", f.name.as_str()));
            let vars = fields
                .iter()
                .map(|f| format_ident!("tg_{}", f.name.as_str()));

            quote! {
                #structname {
                    #(
                        #typenames : #vars
                    ),*
                }
            }
        } else {
            quote! {
                #structname()
            }
        };

        Ok(res)
    }

    fn generate_method(&self, method: &Method) -> Result<TokenStream> {
        let endpoint = &method.name;
        let fn_name = format_ident!("{}", method.name);
        let returntype = self.choose_type(method.returns.as_slice(), false)?;
        let typenames = method
            .fields
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|f| format_ident!("tg_{}", f.name));
        let types = method
            .fields
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|f| self.choose_type(&f.types, !f.required).unwrap());

        let instantiate = self.instantiate_urlencoding_struct(method)?;
        let res = quote! {
            pub async fn #fn_name (&self, #( #typenames: #types ),*) -> Result<#returntype> {
                let form = #instantiate;
                let resp = self.post(#endpoint, form).await?;
                let resp = serde_json::from_value(resp.result)?;
                Ok(resp)
            }
        };

        Ok(res)
    }

    fn choose_type(&self, t: &[String], optional: bool) -> Result<TokenStream> {
        let mytype = if t.len() > 1 {
            self.get_multitype_by_vec(t)?.to_owned()
        } else {
            t[0].clone()
        };

        let nested = is_array(&mytype);

        let res = if nested > 0 {
            let fm = &mytype[ARRAY_OF.len() * nested..];
            let res = type_mapper(&fm);
            let res = format_ident!("{}", res);
            let mut quote = quote!();
            for _ in 0..nested {
                let vec = quote! {
                    Vec<
                };
                quote.extend(vec);
            }
            quote.extend(quote! {
                #res
            });
            for _ in 0..nested {
                let vec = quote! {
                    >
                };
                quote.extend(vec);
            }
            quote
        } else {
            let mytype = type_mapper(&mytype);
            let ret = format_ident!("{}", mytype);
            quote!(#ret)
        };
        if optional {
            Ok(quote! {
                Option<#res>
            })
        } else {
            Ok(res)
        }
    }

    fn generate_use(&self) -> TokenStream {
        quote! {
           use anyhow::Result;
           use reqwest::multipart::{Form, Part};
           use serde::{Deserialize, Serialize};

            use crate::{
                bot::{Bot, Response},
                gen_types::*,
            };
        }
    }

    fn get_multitype_by_vec(&'a self, types: &[String]) -> Result<String> {
        let key = types
            .iter()
            .map(|t| type_without_array(t))
            .collect::<Vec<&str>>()
            .join("");
        let multitypes = self.multitypes.read().expect("failed to lock read access");
        let res = multitypes
            .get(&key)
            .ok_or_else(|| anyhow!("invalid multitype {}", key))?;
        Ok(res.to_owned())
    }

    fn preamble(&self) -> Result<TokenStream> {
        let gen_use = self.generate_use();
        let methods = self
            .spec
            .methods
            .values()
            .filter_map(|m| self.generate_method(&m).ok());
        let opts = self
            .spec
            .methods
            .values()
            .filter_map(|m| self.generate_urlencoding_struct(m).ok());
        Ok(quote! {
            #gen_use

            #( #opts )*

            impl Bot {
                #(
                    #methods
                )*
            }
        })
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
