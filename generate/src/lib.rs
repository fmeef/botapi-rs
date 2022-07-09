use anyhow::Result;
use schema::Spec;
pub(crate) mod schema;

pub struct GenerateTypes(Spec);

impl GenerateTypes {
    pub fn new<T: AsRef<str>>(json: T) -> Result<GenerateTypes> {
        Ok(GenerateTypes(serde_json::from_str(json.as_ref())?))
    }
}
