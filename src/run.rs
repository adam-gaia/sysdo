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

fn format_command(command: &str, args: Option<&[&str]>) -> String {
	format!(
		"{} {}",
		command, args.map_or_else(|| String::new(), |x| x.join(" "))
	).trim_right().to_string()
}

fn bytes_to_str(bytes: &[u8]) -> Result<String> {
	let s = std::str::from_utf8(bytes)?.trim().to_string();
	Ok(s)
}

pub fn run(command: &str, args: Option<&[&str]>) -> Result<String> {
	debug!("Running command: '{}'", format_command(command, args));
	
	let mut cmd_builder = Command::new(command);
	if let Some(args) = args {
		cmd_builder.args(args);
	}
	cmd_builder.env("SSH_TO_AGE_PASSPHRASE","");
	
	let err_message = format!("Failed to run command '{}'", command);
	let output = cmd_builder.output().expect(&err_message);
	let stdout = bytes_to_str(&output.stdout)?;
	debug!("stdout: {}", stdout);
	let stderr = bytes_to_str(&output.stderr)?;
	debug!("stderr: {}", stderr);
	if !output.status.success() {
		let code = output.status.code().unwrap();
		let err_message = format!("Command failed with exit code {}", code);
		bail!(err_message);
	}
	Ok(stdout)
}
