use crate::schema::{Field, Spec};
use crate::{naming::*, schema::Method};
use crate::{util::*, MultiTypes, INPUT_FILE};
use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use quote::{format_ident, quote, ToTokens, __private::TokenStream};
use std::sync::Arc;

/// Generator for telegram api methods. This hould be run after GenerateTypes
pub(crate) struct GenerateMethods<'a> {
    spec: Arc<Spec>,
    multitypes: MultiTypes,
    choose_type: OnceCell<ChooseType<'a>>,
}

impl<'a> GenerateMethods<'a> {
    /// Instantiate GenerateMethods using the json spec and enum type mapping
    pub(crate) fn new(spec: Arc<Spec>, multitypes: MultiTypes) -> Self {
        Self {
            spec,
            multitypes: multitypes.clone(),
            choose_type: OnceCell::new(),
        }
    }

    /// Render a rust source file with api methods
    pub(crate) fn generate_methods(self: &Arc<Self>) -> Result<String> {
        let this = Arc::clone(self);
        let choosetype = ChooseType::new(Arc::clone(&self.spec), move |opts| {
            if is_chatid(opts.types) {
                "i64".to_owned()
            } else if opts.types.len() > 1 {
                this.get_multitype_by_vec(opts.types).unwrap().to_owned()
            } else {
                opts.types[0].clone()
            }
        });
        self.choose_type.set(choosetype).ok().unwrap();
        Ok(self.preamble()?.into_token_stream().to_string())
    }

    fn method_params(&self, method: &Method) -> (Vec<TokenStream>, Vec<TokenStream>) {
        let typenames = method
            .fields
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|f| get_field_name(f))
            .map(|f| format_ident!("{}", f).to_token_stream());
        let types = method
            .fields
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|f| {
                if is_inputfile(&f) {
                    let q = quote! { FileData };
                    is_optional(f, q).to_token_stream()
                } else {
                    if is_str_field(f) {
                        is_optional(f, quote! { &'a str }).to_token_stream()
                    } else {
                        self.choose_type
                            .get()
                            .unwrap()
                            .choose_type_ref(&f.types, None, &f.name, !f.required, || quote! { 'a })
                            .unwrap()
                            .to_token_stream()
                    }
                }
            });

        (typenames.collect(), types.collect())
    }

    fn method_params_req(&self, method: &Method) -> (Vec<TokenStream>, Vec<TokenStream>) {
        let typenames = method
            .fields
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter(|m| m.required)
            .map(|f| get_field_name(f))
            .map(|f| format_ident!("{}", f).to_token_stream());
        let types = method
            .fields
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter(|m| m.required)
            .map(|f| {
                if is_inputfile(&f) {
                    let q = quote! { FileData };
                    is_optional(f, q).to_token_stream()
                } else {
                    self.choose_type
                        .get()
                        .unwrap()
                        .choose_type_ref(&f.types, None, &f.name, !f.required, || quote! { 'a })
                        .unwrap()
                        .to_token_stream()
                }
            });

        (typenames.collect(), types.collect())
    }

    fn generate_call(&self, method: &Method) -> Result<TokenStream> {
        let (typenames, _) = self.method_params(method);
        let returntype = self.choose_type.get().unwrap().choose_type(
            method.returns.as_slice(),
            None,
            &"",
            false,
        )?;
        let name = get_method_name(method);
        let name = format_ident!("{}", name);
        let r = method.fields.as_ref().map_or_else(
            || Vec::new(),
            |f| {
                f.iter()
                    .map(|f| {
                        if !f.required || is_primative(&f.types[0]) || is_inputfile(f) {
                            quote!()
                        } else {
                            quote! { & }
                        }
                    })
                    .collect()
            },
        );
        let res = quote! {
            pub async fn build(self) -> Result<#returntype> {
                self.bot.#name( #( #r self.#typenames ),* ).await
            }
        };

        Ok(res)
    }

    fn generate_call_builder_impl(&self, method: &Method) -> TokenStream {
        let name = get_type_name_str(&method.name);
        let name = format_ident!("Call{}", name);
        let call = self.generate_call(method).unwrap();
        let (names, types) = self.method_params(method);

        let fields = names
            .iter()
            .zip(types.iter())
            .map(|(fieldname, fieldtype)| {
                quote! {
                    pub fn #fieldname(mut self, #fieldname: #fieldtype) -> Self {
                        self.#fieldname = #fieldname;
                        self
                    }
                }
            });

        quote! {
            impl <'a> #name<'a> {
                #( #fields )*
                #call
            }
        }
    }

    fn generate_call_builder(&self, method: &Method) -> TokenStream {
        let name = get_type_name_str(&method.name);
        let name = format_ident!("Call{}", name);
        let i = self.generate_call_builder_impl(method);
        let (names, types) = self.method_params(method);
        quote! {
            pub struct #name <'a>{
                bot: &'a Bot,
                #( #names: #types ),*
            }

            #i
        }
    }

    fn generate_into_form(&self, method: &Method) -> TokenStream {
        let structname = get_type_name_str(&method.name);
        let structname = format_ident!("{}Opts", structname);

        let json_value = method
            .fields
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter(|f| is_json(f))
            .map(|field| {
                let name = get_field_name(field);
                let value = format_ident!("{}", name);
                if field.required {
                    quote! {
                          println!("{} => {}", #name, self.#value);
                        let form = form.text(#name, self.#value);
                    }
                } else {
                    quote! {
                        let form = if let Some(#value) = self.#value {
                            form.text(#name, #value)
                        } else {
                            form
                        };
                    }
                }
            });

        let native_value = method
            .fields
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter(|f| !is_json(f))
            .map(|field| {
                let name = get_field_name(field);
                let value = format_ident!("{}", name);
                if field.required {
                    quote! {
                        println!("{} => {}", #name, self.#value.to_string());
                        let form = form.text(#name, self.#value.to_string());
                    }
                } else {
                    quote! {
                        let form = if let Some(#value) = self.#value {
                            form.text(#name, #value.to_string())
                        } else {
                            form
                        };
                    }
                }
            });

        let lifetime = if method.fields.as_ref().map_or(0, |f| f.len()) == 0
            || method
                .fields
                .as_ref()
                .map_or(false, |f| f.iter().all(|f| no_lifetime(f)))
        {
            quote!()
        } else {
            quote! {
                <'a>
            }
        };

        quote! {
            impl #lifetime #structname #lifetime {
                #[allow(dead_code)]
                fn get_form(self, form: Form) -> Form {
                    #( #json_value )*
                    #( #native_value )*
                    form
                }
            }
        }
    }

    /// Generate a struct for serializing the parameters of a method using
    /// x-www-form-urlencoded
    fn generate_urlencoding_struct(&self, method: &Method) -> Result<TokenStream> {
        let structname = get_type_name_str(&method.name);
        let structname = format_ident!("{}Opts", structname);
        let res = if let Some(fields) = &method.fields {
            let typenames = fields.iter().map(|f| {
                let name = get_field_name(f);
                let name = format_ident!("{}", name);
                if f.required {
                    name.to_token_stream()
                } else {
                    quote! {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        #name
                    }
                }
            });
            let types = fields.iter().map(|f| {
                if is_json(&f) || is_inputfile(&f) {
                    let res = quote! {
                        String
                    };
                    is_optional(f, res)
                } else if is_str_field(f) {
                    is_optional(f, quote! { &'a str })
                } else {
                    self.choose_type
                        .get()
                        .unwrap()
                        .choose_type_ref(&f.types, None, &f.name, !f.required, || quote! { 'a  })
                        .unwrap()
                }
            });

            let lifetime = if fields.iter().all(|f| no_lifetime(f)) {
                quote!()
            } else {
                quote! {
                    <'a>
                }
            };

            quote! {
                #[derive(Serialize, Debug)]
                struct #structname #lifetime {
                    #(
                        #typenames : #types
                    ),*
                }
            }
        } else {
            quote! {
                #[derive(Serialize, Debug)]
                struct #structname();
            }
        };

        Ok(res)
    }

    /// Instantiate structs for all parameters in a method
    fn instantiate_urlencoding_struct(&self, method: &Method) -> Result<TokenStream> {
        let structname = get_type_name_str(&method.name);
        let structname = format_ident!("{}Opts", structname);
        let res = if let Some(fields) = &method.fields {
            if fields.len() == 0 {
                quote!()
            } else {
                let typenames = fields
                    .iter()
                    .map(|f| get_field_name(f))
                    .map(|f| format_ident!("{}", f));
                let vars = fields.iter().map(|f| {
                    let name = get_field_name(f);
                    let name = format_ident!("{}", name);
                    if is_inputfile(&f) {
                        let json_name = format_ident!("{}_json", f.name);
                        quote! {
                            #json_name
                        }
                    } else if is_json(&f) {
                        if f.required {
                            quote! { serde_json::to_string(&#name)? }
                        } else {
                            quote! {
                                if let Some(#name) = #name {
                                    Some(serde_json::to_string(&#name)?)
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        name.to_token_stream()
                    }
                });

                quote! {
                   let form = #structname {
                        #(
                            #typenames : #vars
                        ),*
                    };
                }
            }
        } else {
            quote!()
        };

        Ok(res)
    }

    /// If a method handles uploaded files, generate multipart/form-data code
    fn generate_file_handler(&self, method: &Method) -> TokenStream {
        if let Some(fieldlist) = method
            .fields
            .as_ref()
            .map(|v| v.iter().filter(|f| is_inputfile(&f)))
        {
            let fieldlist = fieldlist.collect::<Vec<&Field>>();
            let res = if fieldlist.len() > 0 {
                quote! { let data = Form::new(); }
            } else {
                quote!()
            };
            let blocks = fieldlist.iter().map(|field| {
                let name = field.name.as_str();
                let typename = format_ident!("{}", name);
                let json_name = format_ident!("{}_json", name);
                if field.required {
                    quote! {
                        let inputfile = #typename.to_inputfile(#name.to_owned());
                        let (data, #json_name) = inputfile.to_form(data)?;
                    }
                } else {
                    quote! {
                        let (data, #json_name) = if let Some(#typename) = #typename {
                            let inputfile = #typename.to_inputfile(#name.to_owned());
                            let (data, #json_name) = inputfile.to_form(data)?;
                            (data, Some(#json_name))
                        } else {
                            (data, None)
                        };
                    }
                }
            });
            quote! {
                #res
                #( #blocks )*
            }
        } else {
            quote!()
        }
    }

    /// Choose what post method to call based on whether we are uploading multipart/form-data
    /// or if we have a method with no parameters (which breaks serde for some reason)
    fn generate_post(&self, method: &Method) -> TokenStream {
        let endpoint = &method.name;
        let inputfile = method
            .fields
            .as_deref()
            .unwrap_or_default()
            .iter()
            .fold(false, |b, f| if is_inputfile(&f) { true } else { b });
        if method.fields.as_deref().unwrap_or_default().len() == 0 {
            quote! {
                self.post_empty(#endpoint).await?
            }
        } else if inputfile {
            quote! {
                self.post_data(#endpoint, form, data).await?
            }
        } else {
            quote! {
                self.post(#endpoint, form).await?
            }
        }
    }

    /// Generate an api method
    fn generate_method(&self, method: &Method) -> Result<TokenStream> {
        let name = get_method_name(method);
        let fn_name = format_ident!("{}", name);
        let returntype = self.choose_type.get().unwrap().choose_type(
            method.returns.as_slice(),
            None,
            &"",
            false,
        )?;

        let (typenames, types) = self.method_params(method);

        let instantiate = self.instantiate_urlencoding_struct(method)?;
        let file_handler = self.generate_file_handler(method);
        let post = self.generate_post(method);
        let comment = method.description.concat().into_comment();
        let res = quote! {
            #comment
            pub async fn #fn_name <'a> (&self, #( #typenames: #types ),*) -> Result<#returntype> {
                #file_handler
                #instantiate
                let resp = #post;
                if resp.ok {
                    let res = resp.result.unwrap_or_default();
                    let resp = serde_path_to_error::deserialize(&res)?;
                    Ok(resp)
                } else {
                    Err(anyhow::anyhow!(resp.description.unwrap_or_default()))
                }
            }
        };

        Ok(res)
    }

    fn generate_builder_method(&self, method: &Method) -> Result<TokenStream> {
        let name = get_method_name(method);
        let fn_name = format_ident!("build_{}", name);
        let returntype = get_type_name_str(&method.name);
        let returntype = format_ident!("Call{}", returntype);

        let (typenames, types) = self.method_params_req(method);

        let nones = method.fields.as_ref().map_or_else(
            || Vec::new(),
            |f| {
                f.iter()
                    .filter(|f| !f.required)
                    .map(|f| {
                        let name = get_field_name(f);
                        let name = format_ident!("{}", name);

                        quote! {
                            #name: None
                        }
                    })
                    .collect()
            },
        );

        let comment = method.description.concat().into_comment();
        let res = quote! {
            #comment
            pub fn #fn_name <'a> (&'a self, #( #typenames: #types ),*) -> #returntype<'a> {
                #returntype {
                    bot: self,
                    #( #typenames , )*
                    #( #nones ),*
                }
            }
        };

        Ok(res)
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

    /// If we find multiple types in one field and we can't make a decision about it,
    /// use the mapping from enum names to types generated from the 'types' phase
    /// to decide what enum to use
    fn get_multitype_by_vec(&self, types: &[String]) -> Result<String> {
        if is_inputfile_types(types) {
            Ok(INPUT_FILE.to_owned())
        } else {
            let key = types
                .iter()
                .map(|t| get_type_name_str(t))
                .collect::<Vec<String>>()
                .join("");
            let multitypes = self.multitypes.read().expect("failed to lock read access");
            let res = multitypes
                .get(&key)
                .ok_or_else(|| anyhow!("invalid multitype {}", key))?;
            Ok(res.to_owned())
        }
    }

    fn preamble(&self) -> Result<TokenStream> {
        let gen_use = self.generate_use();
        let methods = self
            .spec
            .methods
            .values()
            .map(|m| self.generate_method(&m).unwrap());

        let buildermethods = self
            .spec
            .methods
            .values()
            .map(|m| self.generate_builder_method(m).unwrap());

        let opts = self
            .spec
            .methods
            .values()
            .map(|m| self.generate_urlencoding_struct(m).unwrap());

        let calls = self
            .spec
            .methods
            .values()
            .map(|m| self.generate_call_builder(m));

        let forms = self
            .spec
            .methods
            .values()
            .map(|m| self.generate_into_form(m));

        Ok(quote! {
            #gen_use

            #( #opts )*

            #( #calls )*

            #( #forms )*

            impl Bot {
                #(
                    #methods
                )*

                #(
                    #buildermethods
                )*
            }
        })
    }
}
