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
    // pub bind_host: String,
    // pub bind_port: u16,
}

impl Config {
    pub fn from_config<T: AsRef<Path>>(config: T) -> Self {
        File::open(&config)
            .map_err(|e| e.into())
            .and_then(|f| serde_yaml::from_reader(f).map_err(|e| e.into()))
            .unwrap_or_else(|e: failure::Error| {
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
            concurrent_limit: 2,
            check_timeout: 8,
            dead_timeout: 20,
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
            // bind_host: "localhost".to_owned(),
            // bind_port: 3000,
        }
    }
}
