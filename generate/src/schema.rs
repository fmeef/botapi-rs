use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::RwLock;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;

#[cfg(test)]
mod tests {
    use super::{ApxFeedbackArcSet, Spec, Type};
    use std::collections::BTreeSet;

    #[test]
    fn run_to_completion() {
        let json = std::fs::read_to_string("../telegram-bot-api-spec/api.json").unwrap();
        let spec: Spec = serde_json::from_str(&json).unwrap();
        let mut f = ApxFeedbackArcSet::new(&spec);
        let size = f.run().unwrap().len();

        println!("size {}", size);

        // Really conservative measure of success, ensure < 1/8th of all types are boxed
        let vs = spec.iter_types().collect::<BTreeSet<&Type>>().len() / 8;
        assert!(size < vs);
    }
}

/// generate every proper subset of a given size of a set of types
#[allow(dead_code)]
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

/// Convert a spec's types into an iterator over edges in the type digraph
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

/// Failed attempt at a dynamic programming based exact solution to minimum feedback arc set
/// This runs in exponential time + exponential space, which is unusable with the size of
/// telegram's type set. This is just kept around because its cool. Its unused
#[allow(dead_code)]
struct FeedbackArcSet<'a> {
    memo: HashMap<BTreeSet<&'a Type>, usize>,
    edges: BTreeSet<(&'a Type, &'a Type)>,
    vertices: BTreeSet<&'a Type>,
}

/// Approximation for minimum feedback arc set that runs in polynomial time.
/// pirated from SortFAS in http://www.vldb.org/pvldb/vol10/p133-simpson.pdf
pub(crate) struct ApxFeedbackArcSet<'a> {
    edges: BTreeSet<(&'a Type, &'a Type)>,
    vertices: Vec<&'a Type>,
}

impl<'a> ApxFeedbackArcSet<'a> {
    /// Construct a FeedbackArcSet solver from a spec reference
    pub(crate) fn new(spec: &'a Spec) -> Self {
        Self {
            edges: edges_iter(spec)
                .map(|(t, e)| if t.name == "Update" { (e, e) } else { (t, e) })
                .collect::<BTreeSet<(&Type, &Type)>>(),
            vertices: spec
                .iter_types()
                .collect::<BTreeSet<&Type>>()
                .iter()
                .copied()
                .collect::<Vec<&Type>>(),
        }
    }

    /// Check if an edge is pointing backwards in the sorted linear arrangement
    fn is_back_edge(&self, edges: &(&'a Type, &'a Type)) -> Result<bool> {
        let (v, w) = edges;
        let vpos = self
            .vertices
            .iter()
            .position(|i| i == v)
            .ok_or_else(|| anyhow!("cry"))?;
        let wpos = self
            .vertices
            .iter()
            .position(|i| i == w)
            .ok_or_else(|| anyhow!("cry"))?;

        Ok(wpos < vpos)
    }

    /// Count backwards edges and return them
    fn boxed_arcs(&self) -> BTreeSet<(&'a Type, &'a Type)> {
        self.edges
            .iter()
            .copied()
            .filter(|edge| self.is_back_edge(edge).unwrap())
            .collect()
    }

    /// Run the approximation. Return edges we should remove
    pub(crate) fn run(&mut self) -> Result<BTreeSet<(&'a Type, &'a Type)>> {
        for pos in 1..self.vertices.len() + 1 {
            let mut val = 0;
            let mut min = 0;
            let mut loc = pos;
            let v = self.vertices.pop().unwrap();

            for position in (0..(loc - 1)).rev() {
                let w = self
                    .vertices
                    .get(position)
                    .ok_or_else(|| anyhow!("missing vertex at pos {}", position))?;

                if self.edges.contains(&(v, w)) {
                    val -= 1;
                } else if self.edges.contains(&(w, v)) {
                    val += 1;
                }

                if val <= min {
                    min = val;
                    loc = position;
                }
            }

            self.vertices.insert(loc, v);
        }

        Ok(self.boxed_arcs())
    }
}

#[allow(dead_code)]
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
    pub(crate) fn pretty_fields(&self) -> impl Iterator<Item = &Field> {
        self.fields
            .iter()
            .flat_map(|f| f.iter())
            .filter(|f| f.name != "status")
    }

    /// Returns true if a type should be treated as "InputMedia"
    /// used for working with files
    pub(crate) fn is_media(&self) -> bool {
        self.name == "InputMedia"
            || self
                .subtype_of
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
    pub(crate) fn get_type<'a, T: AsRef<str>>(&'a self, name: T) -> Option<&'a Type> {
        self.types.get(name.as_ref())
    }

    /// Gets a method from the spec by name, None if nonexistent
    pub(crate) fn get_method<'a, T: AsRef<str>>(&'a self, name: T) -> Option<&'a Method> {
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
    fn get_subtype_of<'a, T: AsRef<str>>(&'a self, name: T) -> Result<Option<Vec<&'a Type>>> {
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

    /// Check a type by name to see if it should be Box<T> to avoid loops
    pub(crate) fn is_boxed<T: AsRef<str>>(&self, t: T) -> bool {
        let b = self.boxed.read().unwrap();
        b.contains(t.as_ref())
    }

    /// Mark a type by name as boxed
    pub(crate) fn box_type<T: Into<String>>(&self, t: T) -> bool {
        let mut b = self.boxed.write().unwrap();
        b.insert(t.into())
    }

    /// Check if a field (by field name) should be Box<T>
    pub(crate) fn check_parent(&self, parent: &Type, name: &str) -> bool {
        let boxedcheck = format!("{}{}", name, parent.name);
        self.is_boxed(boxedcheck) || parent.name == name
    }
}
