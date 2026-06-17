use std::env;
use std::fs;
use std::path::PathBuf;

use clap::CommandFactory;
use clap_mangen::generate_to;
use rzip::cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("man"));

    fs::create_dir_all(&out_dir)?;

    let cmd = Cli::command();
    generate_to(cmd, &out_dir)?;
    for entry in fs::read_dir(&out_dir)? {
        println!("{}", entry?.path().display());
    }

    Ok(())
}
