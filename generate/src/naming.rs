use crate::{
    schema::{Field, Method, Type},
    util::*,
};

use convert_case::{Case, Casing};
use quote::__private::TokenStream;
use quote::{format_ident, quote, ToTokens};

/// Reserved words in rust as of 2022. Used to avoid generating identifiers that clash with
/// keywords
static RESERVED_WORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn",
];

/// Sanitize an identifier to conform to rust style and avoid using reserved words
pub(crate) fn get_type_name_str<T>(t: &T) -> String
where
    T: AsRef<str>,
{
    let t = type_without_array(t);
    let t = type_mapper(&t);
    let t = t.to_case(Case::UpperCamel);
    if RESERVED_WORDS.contains(&t.as_str()) {
        format!("Tg{t}")
    } else {
        t
    }
}

pub(crate) struct TokenName<T, S> {
    pub(crate) name: S,
    pub(crate) tokens: T,
}

impl<T, S> ToTokens for TokenName<T, S>
where
    T: ToTokens,
    S: AsRef<str> + Clone,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.tokens.to_tokens(tokens)
    }
}

pub(crate) fn type_enum_components<'a, I, S>(
    types: I,
    typename: &'a str,
    noskip: bool,
) -> (
    Vec<TokenName<impl ToTokens + 'a, S>>,
    Vec<TokenName<impl ToTokens + 'a, S>>,
)
where
    I: Iterator<Item = S>,
    S: AsRef<str> + Clone + 'a,
{
    types
        .map(move |v| {
            let u = type_without_array(&v);
            let u = type_mapper(&u);
            let o = get_type_name_str(&v);
            let n = if v.as_ref().starts_with("Array of") {
                format_ident!("{o}Arr")
            } else {
                format_ident!("{}", o)
            };
            let x = if !noskip || u == "String" {
                format_ident!("{u}")
            } else {
                format_ident!("NoSkip{u}")
            };
            let u = if u == type_without_array(&typename) {
                quote! { Box<#x> }
            } else {
                quote! { #x }
            };

            let n = TokenName {
                name: v.clone(),
                tokens: n,
            };

            let u = TokenName { name: v, tokens: u };
            (n, u)
        })
        .unzip()
}

/// Sanitize a type to conform to rust style and avoid using reserved words
pub(crate) fn get_type_name(t: &Type) -> String {
    get_type_name_str(&t.name)
}

/// Sanitize a field name to conform to rust style and avoid using reserved words
pub(crate) fn get_field_name(f: &Field) -> String {
    let f = f.name.to_case(Case::Snake);
    if RESERVED_WORDS.contains(&f.as_str()) {
        format!("tg_{f}")
    } else {
        f
    }
}

/// Santize an method name to conform to rust style and avoid using reserved words
pub(crate) fn get_method_name(m: &Method) -> String {
    let m = m.name.to_case(Case::Snake);
    if RESERVED_WORDS.contains(&m.as_str()) {
        format!("tg_{m}")
    } else {
        m
    }
}
