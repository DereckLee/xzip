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
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, value_name = "ENCODING")]
        encoding: String,
    },
    /// Unpack a ZIP archive using a specific filename encoding.
    Unpack {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, value_name = "ENCODING")]
        encoding: String,
    },
}

fn run(cli: Cli) -> Result<(), RzipError> {
    match cli.command {
        Command::Pack {
            input,
            output,
            encoding,
        } => {
            let encoding = EncodingKind::from_label(&encoding)?;
            pack::pack_path(&input, &output, encoding)
        }
        Command::Unpack {
            input,
            output,
            encoding,
        } => {
            let encoding = EncodingKind::from_label(&encoding)?;
            unpack::unpack_archive(&input, &output, encoding)
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
