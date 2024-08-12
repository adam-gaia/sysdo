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
use std::os::unix::fs::PermissionsExt;
use crate::run::run;
use names::Generator;

pub fn switch(hostname: &str) -> Result<()> {    
    let _ = run("nixos-rebuild", Some(&["switch", "--flake", &format!(".#{}", hostname)]))?;
    Ok(())
}
