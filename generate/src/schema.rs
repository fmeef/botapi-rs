use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Result;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Spec {
    pub(crate) types: HashMap<String, Type>,
    pub(crate) methods: HashMap<String, Method>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Method {
    pub(crate) name: String,
    pub(crate) href: String,
    pub(crate) description: Vec<String>,
    pub(crate) returns: Vec<String>,
    pub(crate) fields: Option<Vec<Field>>,
}

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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) types: Vec<String>,
    pub(crate) required: bool,
    pub(crate) description: String,
}
#[allow(dead_code)]
impl Spec {
    pub(crate) fn get_type<'a, T: AsRef<str>>(&'a self, name: &T) -> Option<&'a Type> {
        self.types.get(name.as_ref())
    }

    pub(crate) fn get_method<'a, T: AsRef<str>>(&'a self, name: &T) -> Option<&'a Method> {
        self.methods.get(name.as_ref())
    }

    pub(crate) fn iter_types<'a>(&'a self) -> impl Iterator<Item = &'a Type> {
        self.types.values()
    }

    pub(crate) fn get_subtypes<'a, T: AsRef<str>>(
        &'a self,
        name: &T,
    ) -> Result<Option<Vec<&'a Type>>> {
        let res = self
            .types
            .get(name.as_ref())
            .map(|t| {
                t.subtypes.as_ref().map(|s| {
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
