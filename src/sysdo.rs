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

use crate::settings::Settings;
mod setup;
use setup::setup;
mod switch;
use switch::switch;
mod status;
use status::status;
use nixgen::label;

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

    pub fn switch(&self) -> Result<()> {
        switch(&self.settings.hostname)?;
        Ok(())
    }

    pub fn status(&self) -> Result<()> {
        status()?;
        Ok(())
    }

    pub fn generation_label(&self) -> Result<()> {
        let label = label()?;
        println!("{}", label);
        Ok(())
    }
}
