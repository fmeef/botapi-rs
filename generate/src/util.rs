use crate::{ARRAY_OF, MULTITYPE_ENUM_PREFIX};
use std::collections::HashSet;

use crate::schema::{Field, Spec, Type};
use anyhow::Result;
use quote::{format_ident, quote, ToTokens, __private::TokenStream};

struct CycleChecker<'a> {
    spec: &'a Spec,
    visited: HashSet<&'a str>,
}

pub(crate) fn get_multitype_name<T>(typename: &T, fieldname: &T) -> String
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

fn is_array<T>(name: &T) -> usize
where
    T: AsRef<str>,
{
    name.as_ref().matches(ARRAY_OF).count()
}

pub(crate) fn choose_type(spec: &Spec, field: &Field, parent: &Type) -> Result<TokenStream> {
    let mytype = &field.types[0];
    let nested = is_array(&mytype);
    let mut checker = CycleChecker::new(spec);
    let t = if nested > 0 {
        let fm = if field.types.len() > 1 {
            get_multitype_name(&parent.name, &field.name)
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
            get_multitype_name(&parent.name, &field.name)
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
    Ok(is_optional(field, t))
}

fn is_optional<T>(field: &Field, tokenstram: T) -> TokenStream
where
    T: ToTokens,
{
    if field.description.starts_with("Optional") {
        let mut start = quote! {
            Option<
        };
        start.extend(tokenstram.to_token_stream());
        start.extend(quote! {
            >
        });
        start
    } else {
        tokenstram.to_token_stream()
    }
}

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
