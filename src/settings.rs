use color_eyre::Result;
use color_eyre::eyre::bail;
use log::debug;
use log::info;
use log::error;
use std::fs;
use std::path::PathBuf;
use std::path::Path;
use directories::BaseDirs;
use std::process::Command;
use jiff::{Zoned, Unit};
use std::env;

#[derive(Debug)]
pub struct Settings {
    pub username: String,
    pub hostname: String,
    pub config_dir: PathBuf,
    pub home_dir: PathBuf,
}

impl Settings {
    pub fn new(username: &str, hostname: &str) -> Result<Self> {
        let Some(base_dirs) = BaseDirs::new() else {
    	    bail!("Unable to get base dirs");
    	};
    	let home_dir = base_dirs.home_dir();
    	let config_dir = base_dirs.config_dir();

        Ok(Self {
          username: username.to_string(), hostname: hostname.to_string(), config_dir: config_dir.to_path_buf(), home_dir: home_dir.to_path_buf(),  
        })
    }
}
