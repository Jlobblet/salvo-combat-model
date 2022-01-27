use crate::config::team::Team;
use crate::config::unit::NamedUnit;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct SimulationConfig {
    pub units: Vec<NamedUnit>,
    pub team_1: Team,
    pub team_2: Team,
}
