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
}

#[derive(Debug, Parser)]
#[clap(version)]
struct Cli {

	#[clap(short, long)]
	dry_run: bool,
	
	#[clap(long)]
	hostname: Option<String>,

	#[clap(long)]
	username: Option<String>,

	#[command(subcommand)]
	command: Commands,
}

fn username(arg: Option<String>) -> Result<String> {
	let username = match arg {
    	Some(username) => username,
    	None => run("id", Some(&["--user", "--name"]))?
    };
    Ok(username)
}

fn hostname(arg: Option<String>) -> Result<String> {
	let hostname = match arg {
    	Some(hostname) => hostname,
    	None => run("hostname", None)?
    };
    Ok(hostname)
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();

	let username = username(args.username.clone())?;
    let hostname = hostname(args.hostname.clone())?;
	let settings = Settings::new(args.dry_run, &username, &hostname)?;

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
	}

	Ok(())
}
