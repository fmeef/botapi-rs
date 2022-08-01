use crate::{
    schema::{Field, Method, Type},
    util::*,
};

use convert_case::{Case, Casing};

static RESERVED_WORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn",
];

pub(crate) fn get_type_name_str<T>(t: &T) -> String
where
    T: AsRef<str>,
{
    let t = type_without_array(t);
    match t {
        "Integer" => "i64".to_owned(),
        "Boolean" => "bool".to_owned(),
        "Float" => "f64".to_owned(),
        _ => {
            let t = t.to_case(Case::UpperCamel);
            if RESERVED_WORDS.contains(&t.as_str()) {
                format!("Tg{}", t)
            } else {
                t
            }
        }
    }
}

pub(crate) fn get_type_name(t: &Type) -> String {
    get_type_name_str(&t.name)
}

pub(crate) fn get_field_name(f: &Field) -> String {
    let f = f.name.to_case(Case::Snake);
    if RESERVED_WORDS.contains(&f.as_str()) {
        format!("tg_{}", f)
    } else {
        f
    }
}

pub(crate) fn get_method_name(m: &Method) -> String {
    let m = m.name.to_case(Case::Snake);
    if RESERVED_WORDS.contains(&m.as_str()) {
        format!("tg_{}", m)
    } else {
        m
    }
}
