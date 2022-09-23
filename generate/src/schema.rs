use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;

/// Serde representation of the json api spec from https://github.com/PaulSonOfLars/telegram-bot-api-spec
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Spec {
    pub(crate) types: HashMap<String, Type>,
    pub(crate) methods: HashMap<String, Method>,
}

/// Serde representation of a json method spec
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Method {
    pub(crate) name: String,
    pub(crate) href: String,
    pub(crate) description: Vec<String>,
    pub(crate) returns: Vec<String>,
    pub(crate) fields: Option<Vec<Field>>,
}

/// Serde representation of a json type spec
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Type {
    pub(crate) name: String,
    pub(crate) href: String,
    pub(crate) description: Vec<String>,
    pub(crate) fields: Option<Vec<Field>>,
    pub(crate) subtypes: Option<Vec<String>>,
    pub(crate) subtype_of: Option<Vec<String>>,
}

/// Serde representation of a json field spec
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) types: Vec<String>,
    pub(crate) required: bool,
    pub(crate) description: String,
}

impl Type {
    /// Returns true if a type should be treated as "InputMedia"
    /// used for working with files
    pub(crate) fn is_media(&self) -> bool {
        self.subtype_of
            .as_ref()
            .map(|v| {
                v.iter()
                    .fold(false, |b, s| if s == "InputMedia" { true } else { b })
            })
            .unwrap_or(false)
    }
}

#[allow(dead_code)]
impl Spec {
    /// Gets a type from the spec by name, None if nonexistent
    pub(crate) fn get_type<'a, T: AsRef<str>>(&'a self, name: &T) -> Option<&'a Type> {
        self.types.get(name.as_ref())
    }

    /// Gets a method from the spec by name, None if nonexistent
    pub(crate) fn get_method<'a, T: AsRef<str>>(&'a self, name: &T) -> Option<&'a Method> {
        self.methods.get(name.as_ref())
    }

    /// returns an Iterator over all types in this spec
    pub(crate) fn iter_types<'a>(&'a self) -> impl Iterator<Item = &'a Type> {
        self.types.values()
    }

    fn get_subtypes_inner(&self, name: &str) -> Option<&[String]> {
        self.types.get(name)?.subtypes.as_deref()
    }

    /// Get all subtypes of a type by name, None if the type is nonexistent, Err if the type
    /// contains nonexistent subtypes
    pub(crate) fn get_subtypes<T: AsRef<str>>(&self, name: T) -> Result<Option<Vec<&Type>>> {
        let subtypes = match self.get_subtypes_inner(name.as_ref()) {
            Some(x) => x,
            None => return Ok(None),
        };

        let mut types = Vec::new();
        for st in subtypes {
            let ty = self.get_type(&st).context("invalid type name")?;
            types.push(ty);
        }

        Ok(Some(types))
    }

    /// Get a list of a type's subtypes, None if the type is nonexistent, Err if any of the
    /// subtypes are nonexistent
    fn get_subtype_of<'a, T: AsRef<str>>(&'a self, name: &T) -> Result<Option<Vec<&'a Type>>> {
        let res = self
            .types
            .get(name.as_ref())
            .map(|t| {
                t.subtype_of.as_ref().map(|s| {
                    s.iter()
                        .map(|st| {
                            self.get_type(&st)
                                .ok_or_else(|| anyhow!("invalid type name"))
                        })
                        .collect::<Result<Vec<&Type>>>()
                })
            })
            .flatten();

        res.map_or(Ok(None), |v| v.map(Some))
    }
}
