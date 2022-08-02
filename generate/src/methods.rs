use crate::schema::Spec;
use crate::{naming::*, schema::Method};
use crate::{util::*, MultiTypes};
use anyhow::{anyhow, Result};
use quote::{format_ident, quote, ToTokens, __private::TokenStream};

use crate::ARRAY_OF;

pub(crate) struct GenerateMethods<'a> {
    spec: &'a Spec,
    multitypes: MultiTypes,
}

impl<'a> GenerateMethods<'a> {
    pub(crate) fn new(spec: &'a Spec, multitypes: MultiTypes) -> Self {
        Self {
            spec,
            multitypes: multitypes.clone(),
        }
    }
    pub(crate) fn generate_methods(&self) -> Result<String> {
        Ok(self.preamble()?.into_token_stream().to_string())
    }

    fn generate_urlencoding_struct(&self, method: &Method) -> Result<TokenStream> {
        let structname = get_type_name_str(&method.name);
        let structname = format_ident!("{}Opts", structname);
        let res = if let Some(fields) = &method.fields {
            let typenames = fields
                .iter()
                .map(|f| get_field_name(f))
                .map(|f| format_ident!("{}", f));
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
        let structname = get_type_name_str(&method.name);
        let structname = format_ident!("{}Opts", structname);
        let res = if let Some(fields) = &method.fields {
            let typenames = fields
                .iter()
                .map(|f| get_field_name(f))
                .map(|f| format_ident!("{}", f));
            let vars = fields
                .iter()
                .map(|f| get_field_name(f))
                .map(|f| format_ident!("{}", f));

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
        let name = get_method_name(method);
        let fn_name = format_ident!("{}", name);
        let returntype = self.choose_type(method.returns.as_slice(), false)?;
        let typenames = method
            .fields
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|f| get_field_name(f))
            .map(|f| format_ident!("{}", f));
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
            .map(|t| get_type_name_str(t))
            .collect::<Vec<String>>()
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
