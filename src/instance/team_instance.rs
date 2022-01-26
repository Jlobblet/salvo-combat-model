use crate::config::simulation_config::SimulationConfig;
use crate::config::unit::Unit;
use eyre::{eyre, Context, Report, Result};
use std::collections::HashMap;

pub struct TeamInstance {
    pub units: Vec<Unit>,
}

impl TryFrom<(HashMap<String, i64>, &HashMap<String, Unit>)> for TeamInstance {
    type Error = Report;

    fn try_from(
        (units, map): (HashMap<String, i64>, &HashMap<String, Unit>),
    ) -> Result<Self, Self::Error> {
        let mut u: Vec<Unit> = Vec::new();
        for (identifier, count) in units {
            let unit = map
                .get(&identifier)
                .ok_or_else(|| eyre!("Could not find unit with identifier {}", identifier))?;
            for _ in 0..count {
                u.push(unit.clone());
            }
        }
        Ok(TeamInstance { units: u })
    }
}

impl TryFrom<SimulationConfig> for (TeamInstance, TeamInstance) {
    type Error = Report;

    fn try_from(value: SimulationConfig) -> Result<Self> {
        let map: HashMap<_, _> = value
            .units
            .into_iter()
            .map(|u| (u.identifier.clone(), u))
            .collect();

        let t1 = TeamInstance::try_from((value.team_1.units, &map))
            .context("Could not create team 1")?;
        let t2 = TeamInstance::try_from((value.team_2.units, &map))
            .context("Could not create team 2")?;
        Ok((t1, t2))
    }
}

impl TeamInstance {
    pub fn is_alive(&self) -> bool {
        self.units.iter().any(|u| u.staying_power > 0.0)
    }

    pub fn attacks(&self) -> Vec<f64> {
        self.units
            .iter()
            .filter(|u| u.staying_power > 0.0)
            .map(|u| u.offensive_firepower)
            .collect()
    }

    pub fn damage(&mut self, attacks: Vec<f64>) {
        for attack in attacks {
            let index = self.units.iter().position(|u| u.staying_power > 0.0);
            if let Some(i) = index {
                let unit = &mut self.units[i];
                let incoming_damage = attack - unit.defensive_firepower;
                if incoming_damage > 0.0 {
                    unit.staying_power -= incoming_damage;
                }
            } else {
                break;
            }
        }
    }
}
