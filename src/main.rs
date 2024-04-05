//! # Lancasim
//!
//! ## Simulate battle outcomes with Lancasim
//!
//! Lancasim is a high-speed simulation environment. It can run many simulations of a battle using [Lanchester's square law](https://en.wikipedia.org/wiki/Lanchester%27s_laws). Results are written concurrently so that file operations do not influence the speed of the simulations. Use lancasim to find the attrition coefficient each team needs to beat the other, given the number of units both teams have.
//!
//! ### Prerequisites
//!
//! - [Cargo](https://www.rust-lang.org/learn/get-started) if you want to compile from source.
//! - A CSV analysis tool like [SandDance for VSCode](https://marketplace.visualstudio.com/items?itemName=msrvida.vscode-sanddance)
//!
//! ### Usage
//!
//! Lancasim is able to run many simulations of a battle, using the [Lanchester's square law](https://en.wikipedia.org/wiki/Lanchester%27s_laws). This is done based on a configuration file like this:
//!
//! ```ini
//! [Blue]
//! ; integer: Start with this amount of units
//! units       = 100
//! ; f32 between 0 and 1: Minimum attrition coefficient for this team
//! min_ac      = 0.1
//! ; f32 between 0 and 1: Maximum attrition coefficient for this team
//! max_ac      = 0.2
//! ; f32 between 0 and 1: Increment for each simulation
//! increment   = 0.01
//!
//! [Red]
//! ; integer: Start with this amount of units
//! units       = 100
//! ; f32 between 0 and 1: Minimum attrition coefficient for this team
//! min_ac      = 0.01
//! ; f32 between 0 and 1: Maximum attrition coefficient for this team
//! max_ac      = 0.05
//! ; f32 between 0 and 1: Increment for each simulation
//! increment   = 0.001
//! ```
//!
//! The simulation environment will run a simulation for all points on a `blue` by `red` matrix, where the attrition coefficients for each team increases incrementally. It will then concurrently write the results to a file in the `results` directory.
//!
//! **Running simulations**
//!
//! Starting lancasim with the `--help` option will show the following:
//!
//! ```text
//!          _       ___   _   _ _____   ___   _____ ________  ___
//!         | |     / _ \ | \ | /  __ \ / _ \ /  ___|_   _|  \/  |
//!         | |    / /_\ \|  \| | /  \// /_\ \\ `--.  | | | .  . |
//!         | |    |  _  || . ` | |    |  _  | `--. \ | | | |\/| |
//!         | |____| | | || |\  | \__/\| | | |/\__/ /_| |_| |  | |
//!         \_____/\_| |_/\_| \_/\____/\_| |_/\____/ \___/\_|  |_/
//!
//!
//!         Simulate battle results with Lancaster simulation.
//!
//! Usage: lancasim [OPTIONS] <file>
//!
//! Arguments:
//!   <file>  Path to the .ini file. Example: ./path/to/file.ini
//!
//! Options:
//!   -f, --full-results  Output full battle diagnostics for each engagement. BEWARE: This is computationally expensive.
//!   -h, --help          Print help
//!   -V, --version       Print version
//! ```

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

/// Main function returns `()` or `LancasterError`
fn main() -> Result<(), LancasterError> {
    let cmd = Command::new("lancasim")
        .version("1.0")
        .about(r#"
         _       ___   _   _ _____   ___   _____ ________  ___
        | |     / _ \ | \ | /  __ \ / _ \ /  ___|_   _|  \/  |
        | |    / /_\ \|  \| | /  \// /_\ \\ `--.  | | | .  . |
        | |    |  _  || . ` | |    |  _  | `--. \ | | | |\/| |
        | |____| | | || |\  | \__/\| | | |/\__/ /_| |_| |  | |
        \_____/\_| |_/\_| \_/\____/\_| |_/\____/ \___/\_|  |_/
                                                              
                                                              
        Simulate battle results with LANCASIM."#)
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
