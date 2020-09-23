use log::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct PhaseConfig {
    pub name: String,
    pub concurrent_limit: u64,
    pub dead_timeout: u64,
    pub check_timeout: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub phases: Vec<PhaseConfig>,
}

impl Config {
    pub fn from_config<T: AsRef<Path>>(config: T) -> Self {
        let f = File::open(&config).unwrap();
        serde_yaml::from_reader(f).unwrap_or_else(|e| {
            warn!(
                "Load config {} failed: {}, use default settings",
                config.as_ref().display(),
                e
            );
            Config::default()
        })
    }
}

impl std::default::Default for Config {
    fn default() -> Self {
        let p1 = PhaseConfig {
            name: "P1".to_string(),
            concurrent_limit: 1,
            check_timeout: 1000,
            dead_timeout: 1000,
        };
        let p2 = PhaseConfig {
            name: "P2".to_string(),
            concurrent_limit: 1,
            check_timeout: 1000,
            dead_timeout: 1000,
        };
        let c1 = PhaseConfig {
            name: "C1".to_string(),
            concurrent_limit: 1,
            check_timeout: 1000,
            dead_timeout: 1000,
        };
        let c2 = PhaseConfig {
            name: "C2".to_string(),
            concurrent_limit: 1,
            check_timeout: 1000,
            dead_timeout: 1000,
        };

        Self {
            phases: vec![p1, p2, c1, c2],
        }
    }
}
