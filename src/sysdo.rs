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
use nixgen::label;
use nixgen::RepoRootConfig;
use crate::settings::Settings;
mod setup;
use setup::setup;
mod status;
use status::status;
use crate::run::run;

#[derive(Debug)]
pub struct Sysdo {
    settings: Settings,
}

impl Sysdo {
    pub fn new(settings: Settings) -> Result<Self> {
        Ok(Self {settings})
    }

    pub fn setup(&self) -> Result<()> {
        setup(&self.settings)?;
        Ok(())
    }

    pub fn build(&self) -> Result<()> {
        let hostname = &self.settings.hostname;
        let _ = run("nixos-rebuild", Some(&["build", "--flake", &format!(".#{}", hostname)]))?;
        Ok(())
    }

    pub fn switch(&self) -> Result<()> {
        let hostname = &self.settings.hostname;
        let label = label(RepoRootConfig::Discover)?;
        let _ = run("nixos-rebuild", Some(&["switch","--profile-name", &label, "--flake", &format!(".#{}", hostname)]))?;     
        Ok(())
    }

    pub fn status(&self) -> Result<()> {
        status()?;
        Ok(())
    }
}
