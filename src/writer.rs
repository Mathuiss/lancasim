use crate::{error::LancasterError, models::SimulationResult};
use std::{
    fs,
    io::Write,
    path::PathBuf,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

pub fn spawn_writer(
    rx: Receiver<SimulationResult>,
    sim_name: String,
    output_dir: PathBuf,
    full_results: bool,
) -> Result<JoinHandle<Result<(), LancasterError>>, LancasterError> {
    let main_file = output_dir.join(format!("{}.csv", sim_name));
    let mut main_file = fs::File::create_new(main_file)?;
    main_file
        .write_all(b"blue_effectiveness,red_effectiveness,engagements,blue_units,red_units\n")?;

    let handler = thread::spawn(move || loop {
        let sim_result = match rx.recv() {
            Ok(o) => o,
            Err(_) => return Err::<(), LancasterError>(LancasterError::from_str("Err")),
        };

        main_file.write_all(
            format!(
                "{},{},{},{},{}\n",
                sim_result.blue_attrition_coefficient(),
                sim_result.red_attrition_coefficient(),
                sim_result.engagements(),
                sim_result.blue_units(),
                sim_result.red_units()
            )
            .as_bytes(),
        )?;

        if full_results {
            let results_file = output_dir.join(format!(
                "b{}-r{}.csv",
                sim_result.blue_attrition_coefficient(),
                sim_result.red_attrition_coefficient()
            ));

            let mut results_file = fs::File::create_new(results_file)?;

            results_file
                .write_all(b"turn,blue_units,red_units,blue_casualties,red_casualties\n")?;

            for res in sim_result.turn_results() {
                results_file.write_all(
                    format!(
                        "{},{},{},{},{}\n",
                        res.turn_nr(),
                        res.blue_units(),
                        res.red_units(),
                        res.blue_casualties(),
                        res.red_casualties()
                    )
                    .as_bytes(),
                )?;
            }
        }
    });

    return Ok(handler);
}
