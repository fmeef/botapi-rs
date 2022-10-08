use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::RwLock;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;

use crate::util::is_primative;
use crate::util::type_mapper;

#[cfg(test)]
mod tests {
    use super::{ApxFeedbackArcSet, FeedbackArcSet, Spec};

    #[test]
    fn run_to_completion() {
        let json = std::fs::read_to_string("../../telegram-bot-api-spec/api.json").unwrap();
        let spec: Spec = serde_json::from_str(&json).unwrap();
        let mut f = ApxFeedbackArcSet::new(&spec);
        let size = f.run().unwrap().len();

        println!("size {}", size);
    }
}

fn all_vertex_sets<'a>(
    sets: &[&'a Type],
    setindex: usize,
    size: usize,
    dataindex: usize,
    data: &mut Vec<&'a Type>,
    out: &mut Vec<Vec<&'a Type>>,
) {
    if dataindex == size {
        out.push(data.clone());
        return;
    }

    if setindex >= sets.len() {
        return;
    }

    if dataindex == data.len() {
        data.push(&sets[setindex]);
    } else if dataindex > data.len() {
        panic!("overflow");
    } else {
        data[dataindex] = &sets[setindex];
    }

    all_vertex_sets(sets, setindex + 1, size, dataindex + 1, data, out);

    all_vertex_sets(sets, setindex + 1, size, dataindex, data, out);
}

fn edges_iter<'a>(spec: &'a Spec) -> impl Iterator<Item = (&'a Type, &'a Type)> {
    spec.iter_types()
        .filter_map(move |t| {
            t.fields.as_ref().map(move |f| {
                f.iter().flat_map(move |field| {
                    field
                        .types
                        .iter()
                        .filter_map(|t2| spec.get_type(&t2))
                        .map(move |t2| (t, t2))
                })
            })
        })
        .flatten()
}

struct FeedbackArcSet<'a> {
    memo: HashMap<BTreeSet<&'a Type>, usize>,
    edges: BTreeSet<(&'a Type, &'a Type)>,
    vertices: BTreeSet<&'a Type>,
}

struct ApxFeedbackArcSet<'a> {
    edges: BTreeSet<(&'a Type, &'a Type)>,
    vertices: HashMap<i64, &'a Type>,
    r_vertices: HashMap<&'a Type, i64>,
    iter_verticies: Vec<(i64, &'a Type)>,
}

impl<'a> ApxFeedbackArcSet<'a> {
    fn new(spec: &'a Spec) -> Self {
        Self {
            edges: edges_iter(spec).collect::<BTreeSet<(&Type, &Type)>>(),
            vertices: (0 as i64..spec.types.len() as i64)
                .zip(spec.iter_types())
                .collect::<HashMap<i64, &Type>>(),
            r_vertices: spec
                .iter_types()
                .zip(0 as i64..spec.types.len() as i64)
                .collect::<HashMap<&Type, i64>>(),
            iter_verticies: (0 as i64..spec.types.len() as i64)
                .zip(spec.iter_types())
                .collect::<Vec<(i64, &Type)>>(),
        }
    }

    fn is_back_edge(&self, edges: &(&'a Type, &'a Type)) -> Result<bool> {
        let (v, w) = edges;
        let v = self
            .r_vertices
            .get(v)
            .ok_or_else(|| anyhow!("bad vertex"))?;
        let w = self
            .r_vertices
            .get(w)
            .ok_or_else(|| anyhow!("bad vertex"))?;
        Ok(w < v)
    }

    fn boxed_arcs(&self) -> BTreeSet<(&'a Type, &'a Type)> {
        self.edges
            .iter()
            .copied()
            .filter(|edge| self.is_back_edge(edge).unwrap())
            .collect()
    }

    fn run(&mut self) -> Result<BTreeSet<(&'a Type, &'a Type)>> {
        for (pos, vertex) in self.iter_verticies.iter() {
            let mut val = 0;
            let mut min = 0;
            let mut loc = *pos;

            for position in (0..loc - 1).rev() {
                let w = self
                    .vertices
                    .get(&position)
                    .ok_or_else(|| anyhow!("missing vertex at pos {}", position))?;

                if self.edges.contains(&(vertex, w)) {
                    val -= 1;
                } else if self.edges.contains(&(w, vertex)) {
                    val += 1;
                }

                if val <= min {
                    min = val;
                    loc = position;
                }
            }

            self.vertices.insert(loc, &vertex);
            self.r_vertices.insert(vertex, loc);
        }

        Ok(self.boxed_arcs())
    }
}

impl<'a> FeedbackArcSet<'a> {
    fn new(spec: &'a Spec) -> Self {
        Self {
            memo: HashMap::new(),
            edges: edges_iter(spec).collect::<BTreeSet<(&Type, &Type)>>(),
            vertices: spec.iter_types().collect::<BTreeSet<&Type>>(),
        }
    }

    fn cutwidth(&self, vertexset: &BTreeSet<&'a Type>, v: &Type) -> usize {
        let tv = BTreeSet::from([v]);
        let tw = BTreeSet::from([v]);
        let w = vertexset.union(&tw).collect::<BTreeSet<_>>();
        let tmp = self
            .vertices
            .difference(vertexset)
            .copied()
            .collect::<BTreeSet<&Type>>();

        let tmp = tmp.difference(&tv).collect::<BTreeSet<_>>();
        self.edges
            .iter()
            .filter(|e| w.contains(&e.0) && tmp.contains(&e.1))
            .count()
    }

    fn run(&mut self) {
        println!("running for graph size {}", self.vertices.len());
        for i in 1..self.vertices.len() {
            let sets = self.all_vertex_sets(i);
            println!("all vertex sets {}", sets.len());
            for s in sets {
                let s = BTreeSet::from_iter(s.iter().copied());
                let v = s
                    .iter()
                    .flat_map(|_| {
                        s.iter()
                            .copied()
                            .map(|v| {
                                let ws = BTreeSet::from([v]);
                                let cw = s.difference(&ws).copied().collect::<BTreeSet<_>>();
                                self.cutwidth(&cw, v)
                            })
                            .max()
                    })
                    .min()
                    .unwrap_or(0);

                self.memo.insert(s, v);
            }
        }
    }

    fn all_vertex_sets(&self, size: usize) -> Vec<Vec<&'a Type>> {
        let mut data = Vec::<&Type>::with_capacity(size);
        let mut out = Vec::<Vec<&Type>>::new();
        let vert = self.vertices.iter().copied().collect::<Vec<&Type>>();
        all_vertex_sets(&vert, 0, size, 0, &mut data, &mut out);
        out
    }
}

/// Serde representation of the json api spec from https://github.com/PaulSonOfLars/telegram-bot-api-spec
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Spec {
    pub(crate) types: HashMap<String, Type>,
    pub(crate) methods: HashMap<String, Method>,
    #[serde(default)]
    boxed: RwLock<HashSet<String>>,
    #[serde(default)]
    min: usize,
}

/// Serde representation of a json method spec
#[allow(dead_code)]
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Method {
    pub(crate) name: String,
    pub(crate) href: String,
    pub(crate) description: Vec<String>,
    pub(crate) returns: Vec<String>,
    pub(crate) fields: Option<Vec<Field>>,
}

/// Serde representation of a json type spec
#[allow(dead_code)]
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

    pub(crate) fn is_boxed<T: AsRef<str>>(&self, t: T) -> bool {
        let b = self.boxed.read().unwrap();
        b.contains(t.as_ref())
    }

    pub(crate) fn box_type<T: Into<String>>(&self, t: T) -> bool {
        let mut b = self.boxed.write().unwrap();
        b.insert(t.into())
    }
}
