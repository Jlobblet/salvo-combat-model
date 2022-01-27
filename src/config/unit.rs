use serde_derive::Deserialize;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Clone, Copy)]
pub struct Unit {
    pub offensive_firepower: f64,
    pub defensive_firepower: f64,
    pub staying_power: f64,
}

impl Unit {
    pub fn is_alive(&self) -> bool {
        self.staying_power > 0.0
    }
}

#[derive(Deserialize, Clone)]
pub struct NamedUnit {
    pub identifier: String,
    #[serde(flatten)]
    pub unit: Unit,
}

impl PartialEq<Self> for NamedUnit {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for NamedUnit {}

impl Hash for NamedUnit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}
