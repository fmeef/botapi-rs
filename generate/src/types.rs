use crate::{schema::Type, MultiTypes, ARRAY_OF, INPUT_FILE, UPDATE};
use anyhow::{anyhow, Ok, Result};
use convert_case::{Case, Casing};
use itertools::Itertools;
use lazy_static::lazy_static;
use quote::{format_ident, quote, ToTokens, __private::TokenStream};

use crate::naming::*;
use crate::schema::{Field, Spec};
use crate::util::*;
use regex::{escape, Regex};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

lazy_static! {
    static ref REGEX_STATUS: Regex = Regex::new(r#""[a-z]+""#).unwrap();
}

/// Generator for the "types" source file
pub(crate) struct GenerateTypes<'a> {
    spec: Arc<Spec>,
    multitypes: MultiTypes,
    choose_type: ChooseType<'a>,
}

impl<'a> GenerateTypes<'a> {
    /// Instantiate a new GenerateTypes using api spec and enum type mapping
    pub(crate) fn new(spec: Arc<Spec>, multitypes: MultiTypes) -> Self {
        Self {
            spec: Arc::clone(&spec),
            multitypes: multitypes.clone(),
            choose_type: ChooseType::new(spec, |opts| {
                let types = opts.types;
                let is_media = opts.is_media;
                let name = opts.name;
                let nested = opts.nested;
                let mytype = &types[0];
                if is_inputfile_types(types) || (is_media && name == "media") {
                    INPUT_FILE.to_owned()
                } else {
                    if is_chatid(types) {
                        "i64".to_owned()
                    } else if types.len() > 1 {
                        get_multitype_name_types(&name, types)
                    } else if nested == 0 {
                        type_mapper(&mytype).to_owned()
                    } else {
                        mytype[ARRAY_OF.len() * nested..].to_owned()
                    }
                }
            }),
        }
    }

    pub(crate) fn generate_types(&self) -> Result<String> {
        Ok(self.preamble()?.into_token_stream().to_string())
    }

    fn preamble(&self) -> Result<TokenStream> {
        let structs = self.spec.types.values().map(|v| {
            if v.fields.as_ref().map(|f| f.len()).unwrap_or(0) > 0 {
                let s = self.generate_struct(&v.name, &v.name, true).unwrap();
                let skip = self
                    .generate_struct(&v.name, &format!("NoSkip{}", v.name), false)
                    .unwrap();
                let fromskip = self.generate_from_skip(v);
                let b = self.generate_builder(&v).unwrap();
                let update = self.generate_update_ext(v);
                let from = self.generate_from_update_ext(v);
                quote! {
                    #s
                    #skip
                    #fromskip
                    #update
                    #from

                    #b
                }
            } else {
                if v.name == INPUT_FILE {
                    self.generate_inputfile_enum()
                } else {
                    let vec = Vec::new();
                    let subtypes = v.subtypes.as_ref().unwrap_or(&vec);
                    let subtypes = subtypes
                        .iter()
                        .filter_map(|v| self.spec.get_type(v))
                        .map(|t| t.name.as_str())
                        .collect::<Vec<&str>>();
                    let statuses = v
                        .subtypes
                        .iter()
                        .flat_map(|v| v.iter())
                        .flat_map(|st| {
                            self.spec
                                .get_type(st)
                                .unwrap()
                                .fields
                                .iter()
                                .flat_map(|v| v.iter())
                                .filter(|field| field.name == "status")
                                .map(|v| {
                                    println!("field {}", v.description);
                                    let d = escape(REGEX_STATUS.as_str());
                                    REGEX_STATUS
                                        .find(&v.description)
                                        .map(|v| &v.as_str()[1..v.as_str().len() - 1])
                                        .expect(&format!("regex {} failed", d))
                                })
                                .map(|v| (v, st.as_str()))
                        })
                        .collect::<HashMap<&str, &str>>();
                    let e = if !statuses.is_empty() {
                        self.generate_enum_internally_tagged(statuses, &v.name, "status")
                            .unwrap()
                    } else {
                        self.generate_enum_str(subtypes.as_slice(), &v.name)
                            .unwrap()
                    };
                    let name = format_ident!("{}", v.name);
                    let methods = self.generate_enum_methods(v);
                    quote! {
                        #e
                        impl #name {
                            #methods
                        }
                    }
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

        let extra = self.generate_method_multitypes()?;
        let uses = self.generate_use()?;
        let tests = self.generate_test();
        let res = quote! {
            #uses
            #( #traits )*
            #( #structs )*
            #( #impls )*
            #enums
            #typeenums
            #extra
            #tests
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
            use std::default::Default;
            use std::borrow::Cow;
        })
    }

    fn get_field_names_ext<'b>(&'b self, t: &'b Type) -> impl Iterator<Item = TokenStream> + 'b {
        t.pretty_fields()
            .filter(|f| f.name != "update_id")
            .map(move |f| {
                let fieldname = get_type_name_str(&f.name);
                let choose = self
                    .choose_type
                    .choose_type(f.types.as_slice(), Some(&t), &f.name, false)
                    .unwrap();
                let name = format_ident!("{}", fieldname);
                quote! {
                    #name(#choose)
                }
            })
    }

    fn has_method(&self, t: &Type, field: &Field) -> bool {
        let common_methods = self.get_common_methods(t);
        if let Some(fields) = t.fields.as_ref() {
            fields.contains(field) || common_methods.contains(field)
        } else {
            common_methods.contains(field)
        }
    }

    /// Generate a special helper type to treat "Update" as an enum
    fn generate_update_ext(&self, t: &Type) -> TokenStream {
        if t.name == UPDATE {
            let fieldnames = self.get_field_names_ext(t);
            let methods = self
                .get_common_methods_recursive_ext(t)
                .iter()
                .filter_map(|field| {
                    let comment = field.description.into_comment();
                    let name = get_field_name(field);
                    let fieldname = format_ident!("get_{}", name);
                    let primative = is_primative(&field.types[0]);

                    let unbox = self
                        .choose_type
                        .choose_type_unbox(field.types.as_slice(), Some(&t), &field.name, false)
                        .unwrap();

                    let is_str = is_str_field(field);
                    let ret = if is_str {
                        quote! { Cow<'a, str> }
                    } else if (field.required && primative) || (!field.required && primative) {
                        unbox
                    } else {
                        quote! { Cow<'a, #unbox> }
                    };

                    let ret = quote! { Option<#ret> };

                    let match_arms = t
                        .pretty_fields()
                        .filter(|f| f.name != "update_id")
                        .filter(|f| {
                            let fieldtype = f.types.first().unwrap();
                            let fieldtype = self.spec.get_type(fieldtype).unwrap();
                            self.has_method(fieldtype, field)
                        })
                        .map(|f| {
                            let t = get_type_name_str(&f.name);
                            let t = format_ident!("{}", t);
                            quote! {
                                Self::#t(ref v) => Some(v.#fieldname())
                            }
                        })
                        .collect_vec();

                    if match_arms.len() == 0 {
                        None
                    } else {
                        let match_arms = match_arms.iter();
                        let mat = if field.required {
                            quote! {
                                match self {
                                    #( #match_arms , )*
                                    _ => None

                                }
                            }
                        } else {
                            quote! {
                                match self {
                                    #( #match_arms , )*
                                    _ => None

                                }.flatten()
                            }
                        };

                        let res = quote! {
                            #comment
                            pub fn #fieldname<'a>(&'a self) -> #ret  {
                                #mat
                            }
                        };
                        Some(res)
                    }
                })
                .collect_vec();

            quote! {
                #[derive(Debug, Clone)]
                pub enum UpdateExt {
                    #( #fieldnames ),*,
                    Invalid
                }

                impl UpdateExt {
                    #( #methods )*
                }
            }
        } else {
            quote!()
        }
    }

    /// Generate From<Update> impl for UpdateExt
    fn generate_from_update_ext(&self, t: &Type) -> TokenStream {
        if t.name == UPDATE {
            let fieldnames = t
                .pretty_fields()
                .filter(|f| f.name != "update_id")
                .map(|f| {
                    let fieldname = get_field_name(&f);
                    let extname = get_type_name_str(&f.name);
                    let name = format_ident!("{}", fieldname);
                    let extname = format_ident!("{}", extname);
                    quote! {
                        if let Some(thing) = update.#name {
                            return Self::#extname(thing);
                        }
                    }
                });

            quote! {
               impl From<Update> for UpdateExt {
                   fn from(update: Update) -> Self {
                        #( #fieldnames )*

                        Self::Invalid
                   }
               }
            }
        } else {
            quote!()
        }
    }

    fn generate_from_skip(&self, t: &Type) -> TokenStream {
        let skipname = format_ident!("NoSkip{}", t.name);
        let name = format_ident!("{}", t.name);
        let fieldnames = t.pretty_fields().map(|f| {
            let fieldname = get_field_name(&f);
            let fieldname = format_ident!("{}", fieldname);
            quote! {
                #fieldname: t.#fieldname
            }
        });

        let into_fieldnames = t.pretty_fields().map(|f| {
            let fieldname = get_field_name(&f);
            let fieldname = format_ident!("{}", fieldname);
            quote! {
                #fieldname: self.#fieldname
            }
        });

        quote! {
           impl From<#skipname> for #name {
               fn from(t: #skipname) -> Self {
                    Self {
                        #( #fieldnames ),*
                    }
               }
           }

            impl Into<#skipname> for #name {
                fn into(self) -> #skipname {
                    #skipname {
                        #( #into_fieldnames ),*
                    }
                }
            }

            impl #skipname {
                pub fn skip(self) -> #name {
                   self.into()
                }
            }

            impl #name {
                pub fn noskip(self) -> #skipname {
                    self.into()
                }
            }
        }
    }

    /// Generate special method for generating multipart/form-data for uploaded files. This is only
    /// generated on "InputFile" which has special treatment in telegram api
    fn generate_inputfile_getter(&self, t: &Type) -> Result<TokenStream> {
        if t.name == INPUT_FILE {
            let q = quote! {
               pub fn to_form(self, data: Form) -> Result<(Form, String)> {
                      match self {
                       InputFile::Bytes(FileBytes { name, bytes: Some(bytes) }) => {
                           let attach = format!("attach://{}", name);
                           let form = data.part(name, Part::bytes(bytes).file_name(""));
                           Ok((form, attach))
                       }
                       InputFile::String(name) => Ok((data, name)),
                       _ => Err(anyhow!("cry")),
                   }
               }
            };
            Ok(q)
        } else {
            Ok(quote!())
        }
    }

    /// Method return types could contain multitypes not generated here normally
    fn generate_method_multitypes(&self) -> Result<TokenStream> {
        let res = self
            .spec
            .methods
            .values()
            .filter(|m| {
                let key = m
                    .returns
                    .iter()
                    .map(|t| get_type_name_str(t))
                    .collect::<Vec<String>>()
                    .join("");

                !self.multitypes.read().unwrap().contains_key(&key)
            })
            .map(|m| &m.returns)
            .map(|r| self.generate_multitype_enum_return(r).unwrap());

        Ok(quote! { #( #res )* })
    }

    /// If a type is a subtype of InputMedia generate multipart/form-data handler method as well
    fn generate_inputmedia_getter(&self, t: &Type) -> Result<TokenStream> {
        if t.is_media() {
            let q = quote! {
               fn to_form(self, data: Form) -> Result<(Form, String)> {
                   match self.media {
                       Some(InputFile::Bytes(FileBytes { name, bytes: Some(bytes) })) => {
                           let attach = format!("attach://{}", name);
                           let form = data.part(name, Part::bytes(bytes));
                           Ok((form, attach))
                       }
                       Some(InputFile::String(name)) => Ok((data, name)),
                       _ => Err(anyhow!("cry")),
                   }
               }
            };
            Ok(q)
        } else {
            Ok(quote!())
        }
    }

    fn generate_multitype_enum_return(&self, types: &[String]) -> Result<TokenStream> {
        if types.len() < 2 {
            Ok(quote!())
        } else if types.iter().all(|v| !is_primative(v)) {
            Ok(quote!())
        } else {
            let res = if !is_inputfile_types(&types) {
                if let Some(name) = self.get_multitype_name_return(&types) {
                    let t = self.generate_enum_str(&types, &name)?;
                    if !is_json_types(types) {
                        let typeiter = types.iter().map(|t| get_type_name_str(t));
                        let types = generate_fmt_display_enum(&name, typeiter);
                        quote! {
                            #t
                            #types
                        }
                    } else {
                        t
                    }
                } else {
                    quote!()
                }
            } else {
                quote!()
            };

            Ok(res)
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

    fn generate_builder(&self, t: &Type) -> Result<TokenStream> {
        let fields = t.pretty_fields().collect_vec();
        let res = if fields.len() > 0 {
            let typename = get_type_name(t);
            let typename_tokens = format_ident!("{}", typename);
            let buildername = format!("{}Builder", typename);
            let buildertokens = format_ident!("{}", buildername);
            let st = self.generate_struct(&t.name, &buildername, true)?;

            let methods = fields.iter().map(|f| {
                let name = f.name.to_case(Case::Snake);
                let name = format_ident!("set_{}", name);
                let fieldname = get_field_name(f);
                let fieldname = format_ident!("{}", fieldname);
                let fieldinst = if f.required {
                    quote! { #fieldname }
                } else {
                    quote! { Some(#fieldname) }
                };

                let fieldtype = self
                    .choose_type
                    .choose_type(&f.types, Some(t), &f.name, false)
                    .unwrap();
                let comment = f.description.into_comment();
                quote! {
                    #comment
                    pub fn #name (mut self, #fieldname: #fieldtype) -> Self {
                        self. #fieldname = #fieldinst;
                        self
                    }
                }
            });

            let instantiate = fields.iter().map(|f| {
                let name = get_field_name(f);
                let name = format_ident!("{}", name);

                quote! {
                    #name: self.#name
                }
            });

            let constructor = self.generate_constructor(&t.name)?;
            quote! {
                #st

                impl #buildertokens {
                    #constructor

                    #( #methods )*

                    pub fn build(self) -> #typename_tokens {
                        #typename_tokens {
                            #( #instantiate ),*
                        }
                    }
                }
            }
        } else {
            quote!()
        };
        Ok(res)
    }

    fn get_multitype_name_return(&self, types: &[String]) -> Option<String> {
        let mut multitypes = self
            .multitypes
            .write()
            .expect("failed to lock write access");
        let key = types
            .iter()
            .map(|t| get_type_name_str(t))
            .collect::<Vec<String>>()
            .join("");
        if is_inputfile_types(types) {
            Some(INPUT_FILE.to_owned())
        } else {
            if let None = multitypes.get(&key) {
                let name = types
                    .iter()
                    .map(|t| get_type_name_str(&t))
                    .collect::<Vec<String>>()
                    .join("");
                multitypes.insert(key, name.clone());
                Some(name)
            } else {
                None
            }
        }
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
    fn generate_enum_internally_tagged(
        &self,
        types: HashMap<&str, &str>,
        name: &str,
        tag: &str,
    ) -> Result<TokenStream> {
        let names_iter = types
            .values()
            .map(|v| get_type_name_str(v))
            .map(|v| format_ident!("{}", v));

        let first_name = types
            .values()
            .map(|v| get_type_name_str(v))
            .map(|v| format_ident!("{}", v))
            .next();
        let types_iter = types
            .values()
            .map(|v| type_without_array(v))
            .map(|v| type_mapper(&v).to_owned())
            .map(|v| format_ident!("{}", v));
        let first_type = types
            .values()
            .map(|v| type_without_array(v))
            .map(|v| type_mapper(&v).to_owned())
            .map(|v| format_ident!("{}", v))
            .next();

        let name = format_ident!("{}", name);
        let default = if let (Some(first_name), Some(first_type)) = (first_name, first_type) {
            quote! {
                impl Default for #name {
                      fn default() -> Self {
                            #name::#first_name(#first_type::default())
                      }
                  }
            }
        } else {
            quote!()
        };

        let tags = types.keys();

        //let enum_methods = self.generate_enum_methods()

        let e = quote! {
            #[derive(Serialize, Deserialize, Debug, Clone)]
            #[serde(tag = #tag)]
            pub enum #name {
                #(
                    #[serde(rename = #tags)]
                    #names_iter(#types_iter)
                ),*
            }
            #default
        };
        Ok(e)
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

        let first_name = types
            .iter()
            .map(|v| get_type_name_str(v))
            .map(|v| format_ident!("{}", v))
            .next();
        let types_iter = types
            .iter()
            .map(|v| type_without_array(v))
            .map(|v| type_mapper(&v).to_owned())
            .map(|v| format_ident!("{}", v));
        let first_type = types
            .iter()
            .map(|v| type_without_array(v))
            .map(|v| type_mapper(&v).to_owned())
            .map(|v| format_ident!("{}", v))
            .next();

        let name = format_ident!("{}", name.as_ref());
        let default = if let (Some(first_name), Some(first_type)) = (first_name, first_type) {
            quote! {
                impl Default for #name {
                      fn default() -> Self {
                            #name::#first_name(#first_type::default())
                      }
                  }
            }
        } else {
            quote!()
        };

        //let enum_methods = self.generate_enum_methods()

        let e = quote! {
            #[derive(Serialize, Deserialize, Debug, Clone)]
            #[serde(untagged)]
            pub enum #name {
                #(
                    #names_iter(#types_iter)
                ),*
            }
            #default
        };
        Ok(e)
    }

    /// For the "InputFile" type genenerate helpers for working with uploaded files and file
    /// references.
    fn generate_inputfile_enum(&self) -> TokenStream {
        let input_file = format_ident!("{}", INPUT_FILE);
        quote! {

            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct FileBytes {
                pub(crate) name: String,
                #[serde(skip, default)]
                pub(crate) bytes: Option<Vec<u8>>
            }

            pub enum FileData {
                Bytes(Vec<u8>),
                String(String)
            }

            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub enum #input_file {
                Bytes(FileBytes),
                String(String)
            }

            impl Default for #input_file {
                fn default() -> Self {
                    #input_file::Bytes(FileBytes {
                        name: "default".to_owned(),
                        bytes: None
                    })
                }
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
        let spec = self.spec.clone();
        let supertype = spec
            .get_type(traitname)
            .ok_or_else(|| anyhow!("type not found"))?;
        let typename = format_ident!("{}", traitname.as_ref());
        let trait_name = format_ident!("Trait{}", traitname.as_ref());
        let functions = self.generate_impl_functions(supertype, false);

        let res = quote! {
            impl #trait_name for #typename {
                    #functions
            }
        };

        Ok(res)
    }

    fn generate_constructor<T>(&self, typename: T) -> Result<TokenStream>
    where
        T: AsRef<str>,
    {
        let t = self
            .spec
            .get_type(&typename.as_ref())
            .ok_or_else(|| anyhow!("type not found"))?;
        if t.fields.as_ref().map_or(0, |f| f.len()) == 0 {
            Ok(quote!())
        } else {
            let fieldtypes = t
                .pretty_fields()
                .filter(|f| f.required && f.name != "type")
                .map(|v| {
                    self.choose_type
                        .choose_type(v.types.as_slice(), Some(&t), &v.name, !v.required)
                        .ok()
                });
            let fieldnames = t
                .pretty_fields()
                .filter(|f| f.required && f.name != "type")
                .map(|f| get_field_name(f))
                .map(|v| format_ident!("{}", v));
            let tgtype = t.fields.as_ref().map_or_else(
                || quote!(),
                |f| {
                    f.iter().find(|n| n.name == "type").map_or_else(
                        || quote!(),
                        |f| {
                            let search = "must be ";
                            let typename = if let Some(index) = f.description.find(search) {
                                &f.description[index + search.len()..]
                            } else {
                                &t.name
                            };

                            if !f.required {
                                quote! { tg_type: Some(#typename.to_owned()) }
                            } else {
                                quote! {
                                    tg_type: #typename.to_owned(),
                                }
                            }
                        },
                    )
                },
            );
            let fieldnames_i = t
                .pretty_fields()
                .filter(|f| f.required && f.name != "type")
                .map(|f| get_field_name(f))
                .map(|v| format_ident!("{}", v));

            let nones = t
                .pretty_fields()
                .filter(|f| !f.required && f.name != "type")
                .map(|f| get_field_name(f))
                .map(|v| format_ident!("{}", v))
                .map(|v| quote! { #v: None });

            let res = quote! {
                pub fn new( #( #fieldnames: #fieldtypes ),* ) -> Self {
                    Self {
                        #tgtype
                        #( #fieldnames_i, )*
                        #( #nones ),*
                    }
                }
            };

            Ok(res)
        }
    }

    fn generate_impl_functions(&self, t: &Type, public: bool) -> TokenStream {
        let methods = t
            .pretty_fields()
            .map(|f| {
                let comment = f.description.into_comment();
                let name = get_field_name(f);
                let fieldname = format_ident!("get_{}", name);
                let fieldname_ref = format_ident!("get_{}_ref", name);
                let returnname = format_ident!("{}", name);
                let primative = is_primative(&f.types[0]);
                let boxed = self.spec.check_parent(t, &f.types[0]);
                let unbox = &self
                    .choose_type
                    .choose_type_unbox(f.types.as_slice(), Some(&t), &f.name, false)
                    .unwrap();
                let is_str = is_str_field(f);

                let access = if is_str && f.required {
                    quote! { Cow::Borrowed(self.#returnname.as_str()) }
                } else if primative {
                    quote! { self.#returnname }
                } else if boxed {
                    quote! { Cow::Borrowed(self.#returnname.as_ref()) }
                } else {
                    quote! { Cow::Borrowed(&self.#returnname) }
                };

                let vaccess = if is_str {
                    quote! { Cow::Borrowed(v.as_str()) }
                } else if boxed {
                    quote! { Cow::Borrowed(v.as_ref()) }
                } else if primative {
                    quote! { *v }
                } else {
                    quote! { Cow::Borrowed(v) }
                };

                let refaccess = if is_str && f.required {
                    quote! { self.#returnname.as_str() }
                } else if primative {
                    quote! { self.#returnname }
                } else if boxed {
                    quote! { self.#returnname.as_ref() }
                } else {
                    quote! { &self.#returnname }
                };

                let refvaccess = if is_str {
                    quote! { v.as_str() }
                } else if boxed {
                    quote! { v.as_ref() }
                } else if primative {
                    quote! { *v }
                } else {
                    quote! { v }
                };

                let ret = if is_str {
                    quote! { Cow<'a, str> }
                } else if (f.required && primative) || (!f.required && primative) {
                    unbox.to_owned()
                } else {
                    quote! { Cow<'a, #unbox> }
                };

                let refret = if is_str {
                    quote! { &'a str }
                } else if (f.required && primative) || (!f.required && primative) {
                    unbox.to_owned()
                } else {
                    quote! { &'a #unbox }
                };

                let ret = if f.required {
                    ret
                } else {
                    quote! { Option<#ret> }
                };

                let refret = if f.required {
                    refret
                } else {
                    quote! { Option<#refret> }
                };

                let public = if public {
                    quote! {
                        pub
                    }
                } else {
                    quote!()
                };

                if f.required {
                    quote! {
                        #comment
                        #public fn #fieldname<'a>(&'a self) -> #ret  {
                            #access
                        }


                        #comment
                        #public fn #fieldname_ref<'a>(&'a self) -> #refret  {
                            #refaccess
                        }
                    }
                } else {
                    quote! {
                        #comment
                        #public fn #fieldname<'a>(&'a self) -> #ret {
                            self.#returnname.as_ref().map(|v| {
                                #vaccess
                            })
                        }


                        #comment
                        #public fn #fieldname_ref<'a>(&'a self) -> #refret {
                            self.#returnname.as_ref().map(|v| {
                                #refvaccess
                            })
                        }
                    }
                }
            })
            .collect_vec();

        quote! {
            #( #methods )*
        }
    }

    pub(crate) fn get_common_methods_recursive_ext(&'a self, t: &'a Type) -> HashSet<&'a Field> {
        let mut set = HashSet::<&Field>::new();

        for field in t.pretty_fields() {
            let mt = field.types.first().unwrap();
            if is_json(field) && is_array(mt) == 0 {
                let t = self
                    .spec
                    .get_type(mt)
                    .expect(&format!("invalid type {}", mt));
                let hashset = self.get_common_methods_recursive(&t);
                set = set.union(&hashset).cloned().collect();
            }
        }
        set.iter().unique_by(|v| v.name.as_str()).cloned().collect()
    }

    pub(crate) fn get_common_methods_recursive(&'a self, t: &'a Type) -> HashSet<&'a Field> {
        let mut set = t.pretty_fields().collect::<HashSet<&Field>>();
        match t.subtypes {
            None => set,
            Some(ref subtypes) => {
                for subtype in subtypes {
                    if let Some(t) = self.spec.get_type(subtype) {
                        println!(
                            "subtype {}",
                            t.subtypes.as_ref().map(|t| t.len()).unwrap_or(0)
                        );
                        let hashset = self.get_common_methods_recursive(t);
                        set = set.intersection(&hashset).cloned().collect();
                    }
                }
                set
            }
        }
    }

    pub(crate) fn get_common_methods(&'a self, t: &Type) -> HashSet<&'a Field> {
        let mut res = HashSet::<&Field>::new();
        if let Some(subtypes) = t.subtypes.as_ref() {
            if let Some(first) = subtypes.first() {
                let first = self.spec.get_type(first).unwrap();
                res = first.pretty_fields().collect();
            }
            for t in subtypes {
                let t = self.spec.get_type(t).unwrap();
                let hashset = self.get_common_methods_recursive(t);
                res = res.intersection(&hashset).cloned().collect();
            }
        }
        res
    }

    fn generate_enum_methods(&self, t: &Type) -> TokenStream {
        if t.is_media() {
            return quote!();
        }
        if let Some(subtypes) = t.subtypes.as_ref() {
            let methods = self
                .get_common_methods(t)
                .iter()
                .map(|f| {
                    let comment = f.description.into_comment();
                    let name = get_field_name(f);
                    let fieldname = format_ident!("get_{}", name);
                    let primative = is_primative(&f.types[0]);

                    let unbox = self
                        .choose_type
                        .choose_type_unbox(f.types.as_slice(), Some(&t), &f.name, false)
                        .unwrap();

                    let is_str = is_str_field(f);
                    let ret = if is_str {
                        quote! { Cow<'a, str> }
                    } else if (f.required && primative) || (!f.required && primative) {
                        unbox
                    } else {
                        quote! { Cow<'a, #unbox> }
                    };

                    let ret = if f.required {
                        ret
                    } else {
                        quote! { Option<#ret> }
                    };

                    let match_arms = subtypes
                        .iter()
                        .map(|t| get_type_name_str(t))
                        .map(|t| format_ident!("{}", t))
                        .map(|t| {
                            quote! {
                                Self::#t(ref v) => v.#fieldname()
                            }
                        });

                    let mat = quote! {
                        match self {
                            #( #match_arms ),*
                        }
                    };

                    quote! {
                        #comment
                        pub fn #fieldname<'a>(&'a self) -> #ret  {
                            #mat
                        }
                    }
                })
                .collect_vec();

            quote! {
                #( #methods )*
            }
        } else {
            quote!()
        }
    }

    /// Generate an impl with getters to allow type erasure
    fn generate_impl<T>(&self, name: &'a T) -> Result<TokenStream>
    where
        T: AsRef<str> + 'a,
    {
        let t = self
            .spec
            .get_type(name)
            .ok_or_else(|| anyhow!("type not found"))?;

        let typename = format_ident!("{}", t.name);
        let methods = self.generate_impl_functions(t, true);
        let form_generator = self.generate_inputmedia_getter(&t)?;
        let inputmedia_generator = self.generate_inputfile_getter(&t)?;
        let constructor = self.generate_constructor(name)?;
        let res = if let Some(subtypes) = t.subtypes.as_ref() {
            let impls = subtypes.iter().map(|v| self.generate_trait_impl(&v).ok());
            quote! {
                #( #impls )*
            }
        } else {
            quote! {
                #[allow(dead_code)]
                impl #typename {
                    #constructor
                    #methods
                    #form_generator
                    #inputmedia_generator
                }

            }
        };
        Ok(res)
    }

    /// Generate a trait impl for a specific type
    fn generate_trait(&self, t: &'a Type) -> Result<TokenStream> {
        let typename = format_ident!("Trait{}", t.name);
        let supertraits = if let Some(subtypes) = t.subtypes.as_ref() {
            let subtypes = subtypes.iter().map(|v| format_ident!("Trait{}", v));
            quote! {
                : #( #subtypes )+*
            }
        } else {
            quote!()
        };

        let methods = t
            .pretty_fields()
            .map(|f| {
                let comment = f.description.into_comment();
                let name = get_field_name(f);
                let fieldname = format_ident!("get_{}", name);
                let fieldname_ref = format_ident!("get_{}_ref", name);
                let primative = is_primative(&f.types[0]);
                let unbox = self
                    .choose_type
                    .choose_type_unbox(f.types.as_slice(), Some(&t), &f.name, false)
                    .unwrap();

                let is_str = is_str_field(f);

                let ret = if is_str {
                    quote! { Cow<'a, str> }
                } else if (f.required && primative) || (!f.required && primative) {
                    unbox.clone()
                } else {
                    quote! { Cow<'a, #unbox> }
                };

                let refret = if is_str {
                    quote! { &'a str }
                } else if (f.required && primative) || (!f.required && primative) {
                    unbox
                } else {
                    quote! { &#unbox }
                };

                let ret = if f.required {
                    ret
                } else {
                    quote! { Option<#ret> }
                };

                let refret = if f.required {
                    refret
                } else {
                    quote! { Option<#refret> }
                };

                if f.required {
                    quote! {
                        #comment
                        fn #fieldname<'a>(&'a self) -> #ret;

                        #comment
                        fn #fieldname_ref<'a>(&'a self) -> #refret;
                    }
                } else {
                    quote! {
                        #comment
                        fn #fieldname<'a>(&'a self) -> #ret;

                        #comment
                        fn #fieldname_ref<'a>(&'a self) -> #refret;
                    }
                }
            })
            .collect_vec();

        let res = quote! {
            trait #typename #supertraits {
                #( #methods )*
            }
        };
        Ok(res)
    }

    fn generate_test(&self) -> TokenStream {
        let tests = self
            .spec
            .types
            .values()
            .filter(|t| t.fields.as_ref().map(|f| f.len()).unwrap_or(0) > 0)
            .map(|t| {
                let name = format_ident!("{}", get_type_name(&t));

                let test_name_msgpack =
                    format_ident!("rmp_serialize_named_{}", t.name.to_case(Case::Snake));
                let test_name_noskip =
                    format_ident!("rmp_serialize_array_{}", t.name.to_case(Case::Snake));
                let test_name_serde =
                    format_ident!("json_serialize_{}", t.name.to_case(Case::Snake));
                quote! {
                    #[test]
                    fn #test_name_msgpack() {
                        let t = #name::default();
                        let ser = rmp_serde::to_vec_named(&t).unwrap();
                        let _: #name = rmp_serde::from_slice(ser.as_slice()).unwrap();
                    }



                    #[test]
                    fn #test_name_noskip() {
                        let t = #name::default();
                        let t = t.noskip();
                        let ser = rmp_serde::to_vec(&t).unwrap();
                        let _: #name = rmp_serde::from_slice(ser.as_slice()).unwrap();
                    }


                    #[test]
                    fn #test_name_serde() {
                        let t = #name::default();
                        let ser = serde_json::to_string(&t).unwrap();
                        println!("{}", ser);
                        let _: #name = serde_json::from_str(&ser).unwrap();
                    }
                }
            });

        quote! {
            #[allow(unused_imports)]
            mod test {
                use super::*;
                use std::default::Default;
                #( #tests )*
            }
        }
    }
    /// Generate a struct based on a type name from the api spec
    fn generate_struct<T>(&self, type_name: T, name: T, serde_skip: bool) -> Result<TokenStream>
    where
        T: AsRef<str>,
    {
        let t = self
            .spec
            .get_type(type_name)
            .ok_or_else(|| anyhow!("type not found"))?;
        let typename = format_ident!("{}", name.as_ref());

        let fieldnames = field_iter(&t, |f| {
            let v = &f.name;
            let fieldname = get_field_name(f);
            let name = format_ident!("{}", fieldname);
            if f.required {
                quote! {
                    #[serde(rename = #v)]
                    #name
                }
            } else {
                if serde_skip {
                    quote! {
                        #[serde(skip_serializing_if = "Option::is_none", rename = #v, default)]
                        #name
                    }
                } else {
                    quote! { #name }
                }
            }
        });
        let fieldtypes = t.pretty_fields().map(|v| {
            self.choose_type
                .choose_type(v.types.as_slice(), Some(&t), &v.name, !v.required)
                .ok()
        });

        let comments = field_iter(&t, |v| v.description.into_comment());
        let struct_comment = if serde_skip {
            t.description.concat().into_comment()
        } else {
            format!(
                "Companion type to {} that doesn't skip fields when serializing. 
                Used for certain deserializers that use arrays to represent struct members",
                t.name
            )
            .into_comment()
        };

        let res = quote! {
            #struct_comment
            #[derive(Serialize, Deserialize, Debug, Default, Clone)]
            pub struct #typename {
                #(
                    #comments
                    #fieldnames: #fieldtypes
                ),*
            }
        };
        Ok(res)
    }
}

mod test {
    #[test]
    fn common_methods() {
        let json = std::fs::read_to_string("../telegram-bot-api-spec/api.json").unwrap();
        let spec: Spec = serde_json::from_str(&json).unwrap();
        let spec = Arc::new(spec);
        let t = spec.get_type("ChatMember").unwrap().clone();
        let types = GenerateTypes::new(Arc::clone(&spec), Arc::new(RwLock::new(HashMap::new())));
        assert!(types.get_common_methods(&t).len() > 0);
    }
}
