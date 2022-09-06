use crate::{schema::Type, MultiTypes, INPUT_FILE};
use anyhow::{anyhow, Result};
use quote::{format_ident, quote, ToTokens, __private::TokenStream};

use crate::naming::*;
use crate::schema::{Field, Spec};
use crate::util::*;

/// Generator for the "types" source file
pub(crate) struct GenerateTypes<'a> {
    spec: &'a Spec,
    multitypes: MultiTypes,
}

impl<'a> GenerateTypes<'a> {
    /// Instantiate a new GenerateTypes using api spec and enum type mapping
    pub(crate) fn new(spec: &'a Spec, multitypes: MultiTypes) -> Self {
        Self {
            spec,
            multitypes: multitypes.clone(),
        }
    }
    pub(crate) fn generate_types(&self) -> Result<String> {
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

    /// Generate use statements for this file
    fn generate_use(&self) -> Result<TokenStream> {
        Ok(quote! {
            use serde::{Deserialize, Serialize};
            use std::fmt;
            use anyhow::{anyhow, Result};
            use reqwest::multipart::{Form, Part};
        })
    }

    /// Generate special method for generating multipart/form-data for uploaded files. This is only
    /// generated on "InputFile" which has special treatment in telegram api
    fn generate_inputfile_getter(&self, t: &Type) -> Result<TokenStream> {
        if t.name == INPUT_FILE {
            let q = quote! {
               pub fn to_form(self, data: Form) -> Result<(Form, String)> {
                   let ser = serde_json::to_string(&self)?;
                   let res = match self {
                       InputFile::Bytes(FileBytes { name, bytes: Some(bytes) }) => {
                           let form = data.part(name, Part::bytes(bytes));
                           Ok(form)
                       }
                       InputFile::String(_) => Ok(data),
                       _ => Err(anyhow!("cry")),
                   }?;
                   Ok((res, ser))
               }
            };
            Ok(q)
        } else {
            Ok(quote!())
        }
    }

    /// If a type is a subtype of InputMedia generate multipart/form-data handler method as well
    fn generate_inputmedia_getter(&self, t: &Type) -> Result<TokenStream> {
        if t.is_media() {
            let q = quote! {
               fn to_form(self, data: Form) -> Result<(Form, String)> {
                   let ser = serde_json::to_string(&self)?;
                   let res = match self.media {
                       Some(InputFile::Bytes(FileBytes { name, bytes: Some(bytes) })) => {
                           let form = data.part(name, Part::bytes(bytes));
                           Ok(form)
                       }
                       Some(InputFile::String(_)) => Ok(data),
                       _ => Err(anyhow!("cry")),
                   }?;
                   Ok((res, ser))
               }
            };
            Ok(q)
        } else {
            Ok(quote!())
        }
    }

    /// If we can't chose a specific type when a field has multiple types, generate an enum with
    /// all types
    fn generate_multitype_enums(&self) -> Result<TokenStream> {
        let mut tokens = quote!();

        for jsontype in self.spec.types.values() {
            if let Some(fields) = jsontype.fields.as_ref() {
                for field in fields {
                    if field.types.len() > 1 {
                        if !is_inputfile(&field) {
                            if let Some(name) = self.get_multitype_name(&field) {
                                let t = self.generate_enum_str(&field.types, &name)?;
                                tokens.extend(t);

                                if !is_json(&field) {
                                    let typeiter = field.types.iter().map(|t| get_type_name_str(t));
                                    let t = generate_fmt_display_enum(&name, typeiter);
                                    tokens.extend(t);
                                }
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
                        if !is_inputfile(&field) {
                            if let Some(name) = self.get_multitype_name(&field) {
                                let t = self.generate_enum_str(&field.types, &name)?;
                                tokens.extend(t);

                                if !is_json(&field) {
                                    let typeiter = field.types.iter().map(|t| get_type_name_str(t));
                                    let t = generate_fmt_display_enum(&name, typeiter);
                                    tokens.extend(t);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(tokens)
    }

    /// Helper method for generating a name for a multitype enum while storing it in the mapping
    /// for later use by methods generator
    fn get_multitype_name(&self, field_name: &Field) -> Option<String> {
        let mut multitypes = self
            .multitypes
            .write()
            .expect("failed to lock write access");
        let key = field_name
            .types
            .iter()
            .map(|t| get_type_name_str(t))
            .collect::<Vec<String>>()
            .join("");
        if is_inputfile(field_name) {
            Some(INPUT_FILE.to_owned())
        } else {
            if let None = multitypes.get(&key) {
                let name = get_multitype_name(field_name);
                multitypes.insert(key, name.clone());
                Some(name)
            } else {
                None
            }
        }
    }

    /// Generate an enum with custom types
    fn generate_enum_str<N, I>(&self, types: &[I], name: &N) -> Result<TokenStream>
    where
        N: AsRef<str>,
        I: AsRef<str>,
    {
        let names_iter = types
            .iter()
            .map(|v| get_type_name_str(v))
            .map(|v| format_ident!("{}", v));
        let types_iter = types
            .iter()
            .map(|v| type_without_array(v))
            .map(|v| type_mapper(&v))
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

    /// For the "InputFile" type genenerate helpers for working with uploaded files and file
    /// references.
    fn generate_inputfile_enum(&self) -> TokenStream {
        let input_file = format_ident!("{}", INPUT_FILE);
        quote! {

            #[derive(Serialize, Deserialize, Debug)]
            pub struct FileBytes {
                pub(crate) name: String,
                #[serde(skip, default)]
                pub(crate) bytes: Option<Vec<u8>>
            }

            pub enum FileData {
                Bytes(Vec<u8>),
                String(String)
            }

            #[derive(Serialize, Deserialize, Debug)]
            pub enum #input_file {
                Bytes(FileBytes),
                String(String)
            }

            impl FileData {
                pub fn to_inputfile(self, name: String) -> #input_file {
                     match self {
                        FileData::Bytes(bytes) => #input_file::Bytes(FileBytes {
                            name: name,
                            bytes: Some(bytes),
                        }),
                        FileData::String(name) => #input_file::String(name),
                    }
                }
            }
        }
    }

    /// For subtypes, generate traits and trait bounds to allow generics to work
    fn generate_trait_impl<T>(&self, traitname: &T) -> Result<TokenStream>
    where
        T: AsRef<str>,
    {
        let supertype = self
            .spec
            .get_type(traitname)
            .ok_or_else(|| anyhow!("type not found"))?;
        let typename = format_ident!("{}", traitname.as_ref());
        let fieldnames = field_iter_str(&supertype, |v| format_ident!("get_{}", v));
        let returnnames = field_iter_str(&supertype, |v| format_ident!("{}", v));
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

    /// Generate an impl with getters to allow type erasure
    fn generate_impl<T>(&self, name: &T) -> Result<TokenStream>
    where
        T: AsRef<str>,
    {
        let t = self
            .spec
            .get_type(name)
            .ok_or_else(|| anyhow!("type not found"))?;

        let typename = format_ident!("{}", t.name);
        let fieldnames = field_iter_str(&t, |v| format_ident!("get_{}", v));
        let returnnames = field_iter_str(&t, |v| format_ident!("{}", v));

        let fieldtypes = field_iter(&t, |v| choose_type(&self.spec, v, &t).ok());
        let comments = field_iter(&t, |v| v.description.into_comment());

        let form_generator = self.generate_inputmedia_getter(&t)?;
        let inputmedia_generator = self.generate_inputfile_getter(&t)?;
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

                    #form_generator
                    #inputmedia_generator
                }

            }
        };
        Ok(res)
    }

    /// Generate a trait impl for a specific type
    fn generate_trait(&self, t: &Type) -> Result<TokenStream> {
        let typename = format_ident!("Trait{}", t.name);
        let fieldnames = field_iter_str(&t, |v| format_ident!("get_{}", v));
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

    /// Generate a struct based on a type name from the api spec
    fn generate_struct<T>(&self, name: &T) -> Result<TokenStream>
    where
        T: AsRef<str>,
    {
        let t = self
            .spec
            .get_type(name)
            .ok_or_else(|| anyhow!("type not found"))?;
        let typename = format_ident!("{}", t.name);
        let fieldnames = field_iter_str(&t, |v| format_ident!("{}", v));
        let serdenames = t
            .fields
            .iter()
            .flat_map(|f| f.iter().map(|v| v.name.as_str()));
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
