mod config;
mod instance;
mod model;

use crate::config::simulation_config::SimulationConfig;
use crate::model::simulate;
use eyre::{Context, Result};
use std::fs;
use std::time::Instant;

fn main() -> Result<()> {
    let start = Instant::now();
    let config = fs::read_to_string("config.toml").context("Could not read config file")?;
    let simulation_config: SimulationConfig =
        toml::from_str(&config).context("Could not create simulation config")?;
    simulate(simulation_config).context("Could not run simulation")?;
    let end = Instant::now();
    println!("{}us", (end - start).as_micros());
    Ok(())
}
