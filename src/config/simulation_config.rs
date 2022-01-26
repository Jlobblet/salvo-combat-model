use crate::config::team::Team;
use crate::config::unit::Unit;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct SimulationConfig {
    pub units: Vec<Unit>,
    pub team_1: Team,
    pub team_2: Team,
}
