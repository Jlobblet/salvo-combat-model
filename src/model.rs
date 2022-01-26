use crate::config::simulation_config::SimulationConfig;
use crate::instance::team_instance::TeamInstance;
use eyre::Result;

fn ply(t1: &mut TeamInstance, t2: &mut TeamInstance) -> Result<()> {
    let a1 = t1.attacks();
    let a2 = t2.attacks();
    t2.damage(a1);
    t1.damage(a2);
    Ok(())
}

pub fn simulate(config: SimulationConfig) -> Result<()> {
    let (mut t1, mut t2): (TeamInstance, TeamInstance) = config.try_into()?;
    while t1.is_alive() && t2.is_alive() {
        ply(&mut t1, &mut t2)?;
    }
    let winner = match (t1.is_alive(), t2.is_alive()) {
        (false, false) => "none",
        (true, false) => "team 1",
        (false, true) => "team 2",
        (true, true) => "stalemate?",
    };
    println!("{}", winner);
    Ok(())
}
