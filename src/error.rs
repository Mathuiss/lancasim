use std::sync::mpsc::{RecvError, SendError};

use crate::models::SimulationResult;

#[derive(Debug)]
pub struct LancasterError {
    msg: String,
}

impl LancasterError {
    pub fn from(msg: String) -> Self {
        Self { msg }
    }

    pub fn from_str(msg: &str) -> Self {
        Self {
            msg: String::from(msg),
        }
    }
}

impl From<SendError<SimulationResult>> for LancasterError {
    fn from(value: SendError<SimulationResult>) -> Self {
        Self::from(format!(
            "No writer thread. Failed to write simulation results: {:?}",
            value
        ))
    }
}

impl From<RecvError> for LancasterError {
    fn from(_: RecvError) -> Self {
        Self::from_str("Simulation stoppped. Can't read output.")
    }
}

impl From<ConfigError> for LancasterError {
    fn from(value: ConfigError) -> Self {
        Self { msg: value.msg() }
    }
}

impl From<std::io::Error> for LancasterError {
    fn from(value: std::io::Error) -> Self {
        Self {
            msg: format!("{:?}", value),
        }
    }
}

pub struct ConfigError {
    msg: String,
}

impl ConfigError {
    pub fn from(msg: String) -> Self {
        Self { msg }
    }

    pub fn from_str(msg: &str) -> Self {
        Self {
            msg: String::from(msg),
        }
    }

    pub fn msg(&self) -> String {
        self.msg.clone()
    }
}

impl From<ini::Error> for ConfigError {
    fn from(_: ini::Error) -> Self {
        Self {
            msg: format!("Could not find file"),
        }
    }
}

impl From<std::num::ParseIntError> for ConfigError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self {
            msg: format!("{:?}", value),
        }
    }
}

impl From<std::num::ParseFloatError> for ConfigError {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self {
            msg: format!("{:?}", value),
        }
    }
}
