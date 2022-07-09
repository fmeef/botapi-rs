use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Spec {
    pub(crate) types: HashMap<String, Type>,
    pub(crate) methods: HashMap<String, Method>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Method {
    pub(crate) name: String,
    pub(crate) href: String,
    pub(crate) description: Vec<String>,
    pub(crate) returns: Vec<String>,
    pub(crate) fields: Option<Vec<Field>>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Type {
    pub(crate) name: String,
    pub(crate) href: String,
    pub(crate) description: Vec<String>,
    pub(crate) fields: Option<Vec<Field>>,
    pub(crate) subtypes: Option<Vec<String>>,
    pub(crate) subtype_of: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) types: Vec<String>,
    pub(crate) required: bool,
    pub(crate) description: String,
}
