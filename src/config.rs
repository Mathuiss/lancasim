use crate::error::ConfigError;
use ini::{Ini, Properties};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TeamConfig {
    units: i64,
    min_ac: f32,
    max_ac: f32,
    increment: f32,
}

impl TeamConfig {
    pub fn units(&self) -> i64 {
        self.units
    }

    pub fn min_ac(&self) -> f32 {
        self.min_ac
    }

    pub fn max_ac(&self) -> f32 {
        self.max_ac
    }

    pub fn increment(&self) -> f32 {
        self.increment
    }
}

/// TODO: Check that max_ac > min_ac
impl TeamConfig {
    pub fn from(team: &Properties) -> Result<Self, ConfigError> {
        let min_ac = team
            .get("min_ac")
            .ok_or(ConfigError::from_str("min_ac not found"))?
            .parse::<f32>()?;

        let max_ac = team
            .get("max_ac")
            .ok_or(ConfigError::from_str("max_ac not found"))?
            .parse::<f32>()?;

        let increment = team
            .get("increment")
            .ok_or(ConfigError::from_str("increment not found"))?
            .parse::<f32>()?;

        Ok(Self {
            units: team
                .get("units")
                .ok_or(ConfigError::from_str("units not found"))?
                .parse::<i64>()?,
            min_ac: if min_ac > 0.0 {
                min_ac
            } else {
                return Err(ConfigError::from(format!(
                    "Value min_ac must be larger thgan 0"
                )));
            },
            max_ac: if max_ac > 0.0 {
                max_ac
            } else {
                return Err(ConfigError::from(format!(
                    "Value max_ac must be larger thgan 0"
                )));
            },
            increment: if increment > 0.0 {
                increment
            } else {
                return Err(ConfigError::from(format!(
                    "Value increment must be larger thgan 0"
                )));
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct StartupConfig {
    blue_team: TeamConfig,
    red_team: TeamConfig,
}

impl StartupConfig {
    pub fn blue_team(&self) -> &TeamConfig {
        &self.blue_team
    }

    pub fn red_team(&self) -> &TeamConfig {
        &self.red_team
    }
}

impl StartupConfig {
    pub fn from(file: &PathBuf) -> Result<Self, ConfigError> {
        let cfg = Ini::load_from_file(file)?;

        let blue = cfg
            .section(Some("Blue"))
            .ok_or(ConfigError::from_str("Section [Blue] not found"))?;
        let red = cfg
            .section(Some("Red"))
            .ok_or(ConfigError::from_str("Section [Red] not found"))?;

        Ok(Self {
            blue_team: TeamConfig::from(blue)?,
            red_team: TeamConfig::from(red)?,
        })
    }
}
