use serde_derive::Deserialize;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Clone)]
pub struct Unit {
    pub identifier: String,
    pub offensive_firepower: f64,
    pub defensive_firepower: f64,
    pub staying_power: f64,
}

impl PartialEq<Self> for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for Unit {}

impl Hash for Unit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}
