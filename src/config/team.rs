use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Team {
    pub units: HashMap<String, i64>,
}
