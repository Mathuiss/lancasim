use std::sync::mpsc::{SendError, Sender};

use crate::{
    config::StartupConfig,
    error::LancasterError,
    models::{Battlefield, SimulationResult, TurnResult},
};

pub fn run_all(
    startup_config: &StartupConfig,
    tx: Sender<SimulationResult>,
) -> Result<(), LancasterError> {
    let blue_min = startup_config.blue_team().min_ac();
    let blue_max = startup_config.blue_team().max_ac();
    let blue_inc = startup_config.blue_team().increment();

    let red_min = startup_config.red_team().min_ac();
    let red_max = startup_config.red_team().max_ac();
    let red_inc = startup_config.red_team().increment();

    let blue_incs = ((blue_max - blue_min) / blue_inc).abs() as i64;
    let red_incs = ((red_max - red_min) / red_inc).abs() as i64;
    let total_simulations = blue_incs * red_incs;

    println!("Running {} simulations...", total_simulations);

    let mut blue_ac = blue_min;

    while blue_ac <= blue_max {
        let mut red_ac = red_min;
        while red_ac <= red_max {
            self::run_simulation(
                tx.clone(),
                startup_config.blue_team().units(),
                startup_config.red_team().units(),
                blue_ac,
                red_ac,
            )?;

            red_ac += red_inc;
        }

        blue_ac += blue_inc;
    }

    Ok(())
}

fn run_simulation(
    tx: Sender<SimulationResult>,
    blue_units: i64,
    red_units: i64,
    blue_ac: f32,
    red_ac: f32,
) -> Result<(), SendError<SimulationResult>> {
    let mut turn_nr = 1;
    let mut battlefield = Battlefield::new(blue_units, red_units, blue_ac, red_ac);
    let mut turn_results: Vec<TurnResult> = Vec::new();
    turn_results.push(TurnResult::new(
        0,
        battlefield.blue_units(),
        battlefield.red_units(),
        0,
        0,
    ));

    while battlefield.blue_units() > 0 && battlefield.red_units() > 0 {
        let (blue_casualties, red_casualties) = battlefield.simulate_turn();
        let turn_result = TurnResult::new(
            turn_nr,
            battlefield.blue_units(),
            battlefield.red_units(),
            blue_casualties,
            red_casualties,
        );

        turn_results.push(turn_result);

        turn_nr += 1;
    }

    let simulation_result = SimulationResult::new(
        blue_ac,
        red_ac,
        turn_nr,
        battlefield.blue_units(),
        battlefield.red_units(),
        turn_results,
    );
    tx.send(simulation_result)?;

    Ok(())
}
