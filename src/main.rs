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

use clap::{Parser, Args, Subcommand, ValueEnum};

mod settings;
use settings::Settings;
mod run;
use run::run;
mod sysdo;
use sysdo::Sysdo;

#[derive(Debug, Subcommand)]
enum Commands {
	/// Run initial setup on a new machine
	Setup,

	/// Run nixos-rebuild switch
	Switch,

	/// Get a quick looko at the status of the system
	Status,

	/// Generate a name for a NixOS Generation
	GenerationLabel,
}

#[derive(Debug, Parser)]
#[clap(version)]
struct Cli {
	#[clap(long)]
	hostname: Option<String>,

	#[clap(long)]
	username: Option<String>,

	#[command(subcommand)]
	command: Commands,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();

    let username = match args.username {
    	Some(username) => username,
    	None => run("id", Some(&["--user", "--name"]))?
    };

    let hostname = match args.hostname {
    	Some(hostname) => hostname,
    	None => run("hostname", None)?
    };
 
    debug!("{}@{}", username, hostname);    
	let settings = Settings::new(&username, &hostname)?;

	let app = Sysdo::new(settings)?;
	match args.command {
		Commands::Setup => {
			app.setup()?;
		}
		Commands::Switch => {
		  app.switch()?;
		}
		Commands::Status => {
			app.status()?;
		}		
		Commands::GenerationLabel => {
			app.generation_label()?;
		}
	}

	Ok(())
}
