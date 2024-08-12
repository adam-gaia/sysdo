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

fn generation_label() -> Result<()> {
    let label = label()?;
    println!("{}", label);
    Ok(())
}

fn init(args: &Cli) -> Result<Sysdo> {	
	let username = username(args.username.clone())?;
    let hostname = hostname(args.hostname.clone())?;
	let settings = Settings::new(&username, &hostname)?;
	let app = Sysdo::new(settings)?;
	Ok(app)
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();
	match args.command {
		Commands::Setup => {
			let app = init(&args)?;
			app.setup()?;
		}
		Commands::Switch => {
			let app = init(&args)?;
			app.switch()?;
		}
		Commands::Status => {
			let app = init(&args)?;
			app.status()?;
		}		
		Commands::GenerationLabel => {
			generation_label()?;
		}
	}

	Ok(())
}
