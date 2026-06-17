use std::path::PathBuf;

use clap::{Parser, Subcommand};

use rzip::codec::EncodingKind;
use rzip::error::RzipError;
use rzip::{pack, unpack};

#[derive(Parser, Debug)]
#[command(
    name = "rzip",
    version,
    about = "ZIP tool with explicit filename encoding control"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Pack directory or file into a ZIP archive.
    Pack {
        #[arg(short = 'i', long)]
        input: PathBuf,
        #[arg(short = 'o', long)]
        output: PathBuf,
        #[arg(short = 'e', long, value_name = "ENCODING", default_value = "utf-8")]
        encoding: String,
        #[arg(short = 'r', long)]
        recursive: bool,
        #[arg(long = "include", value_name = "GLOB")]
        include: Vec<String>,
        #[arg(long = "exclude", value_name = "GLOB")]
        exclude: Vec<String>,
    },
    /// Unpack a ZIP archive using a specific filename encoding.
    Unpack {
        #[arg(short = 'i', long)]
        input: PathBuf,
        #[arg(short = 'o', long)]
        output: PathBuf,
        #[arg(short = 'e', long, value_name = "ENCODING", default_value = "utf-8")]
        encoding: String,
        #[arg(long = "include", value_name = "GLOB")]
        include: Vec<String>,
        #[arg(long = "exclude", value_name = "GLOB")]
        exclude: Vec<String>,
    },
}

fn run(cli: Cli) -> Result<(), RzipError> {
    match cli.command {
        Command::Pack {
            input,
            output,
            encoding,
            recursive,
            include,
            exclude,
        } => {
            let encoding = EncodingKind::from_label(&encoding)?;
            pack::pack_path(&input, &output, encoding, recursive, &include, &exclude)
        }
        Command::Unpack {
            input,
            output,
            encoding,
            include,
            exclude,
        } => {
            let encoding = EncodingKind::from_label(&encoding)?;
            unpack::unpack_archive(&input, &output, encoding, &include, &exclude)
        }
    }
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
