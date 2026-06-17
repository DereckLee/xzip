use clap::Parser;

use rzip::cli::{Cli, Command};
use rzip::codec::EncodingKind;
use rzip::error::RzipError;
use rzip::{pack, unpack};

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
