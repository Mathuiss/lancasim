use crate::{error::ConfigError, writer::spawn_writer};
use clap::{Arg, ArgAction, Command};
use config::StartupConfig;
use error::LancasterError;
use models::SimulationResult;
use std::{env, fs, path::PathBuf, sync::mpsc::channel};

mod config;
mod error;
mod lancasim;
mod models;
mod writer;

/// Some type of explanation of **lancasim**
fn main() -> Result<(), LancasterError> {
    let cmd = Command::new("lancasim")
        .version("0.1")
        .about("Simulate battle results with Lancaster simulation.")
        .arg(
            Arg::new("file")
                .required(true)
                .help("Path to the .ini file. Example: ./path/to/file.ini"),
        )
        .arg(
            Arg::new("fullresults")
                .required(false)
                .short('f')
                .long("full-results")
                .action(ArgAction::SetTrue)
                .help("Output full battle diagnostics for each engagement. BEWARE: This is computationally expensive."),
        );

    // Get cli arg
    let matches = cmd.get_matches();
    let file = matches.get_one::<String>("file").unwrap();
    let full_results = matches.get_one::<bool>("fullresults").unwrap();

    // Load config
    let file_path = get_file_path(file)?;
    let startup_config = StartupConfig::from(&file_path)?;

    // Get simulation name
    let sim_name = String::from(
        file_path
            .file_name()
            .ok_or(ConfigError::from(format!(
                "Problem with path: {:?}",
                file_path
            )))?
            .to_str()
            .ok_or(ConfigError::from(format!(
                "Problem with path: {:?}",
                file_path
            )))?,
    )
    .replace(".ini", "");

    // Check or create data structure of the output directory
    let output_dir = check_data_folder(&sim_name)?;

    // Setup mpsc channels
    let (tx, rx) = channel::<SimulationResult>();

    // Spawn results writer thread
    let handle = spawn_writer(rx, sim_name, output_dir.clone(), *full_results)?;

    // Run lancaster simulations
    lancasim::run_all(&startup_config, tx)?;

    println!(
        "Writing simulation results to: {}",
        output_dir.to_string_lossy()
    );

    _ = handle.join();

    Ok(())
}

/// Gets the full path of the file, parsed from cli arguments.
fn get_file_path(file: &String) -> std::io::Result<PathBuf> {
    Ok(env::current_dir()?.join(PathBuf::from(file)))
}

/// Checks if there is a data folder by appending the *current working directory* with `results`.
/// It then also tries to create a directory with the name of the current test inside of the *results* directory.
fn check_data_folder(test_name: &String) -> std::io::Result<PathBuf> {
    let data_dir = env::current_dir()?.join(PathBuf::from("results"));
    let results_dir = data_dir.join(PathBuf::from(test_name));

    if !data_dir.exists() {
        fs::create_dir(data_dir)?;
    }

    if !results_dir.exists() {
        fs::create_dir(&results_dir)?;
    }

    Ok(results_dir)
}
