use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "rzip",
    version,
    about = "ZIP tool with explicit filename encoding control",
    long_about = "Pack and unpack ZIP archives with configurable filename encoding.\n\
                  Defaults to utf-8 when --encoding is omitted; use -e gbk (or cp936) \
                  for archives created on zh_CN Windows systems."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Pack a directory or file into a ZIP archive.
    Pack {
        /// Input file or directory to archive.
        #[arg(short = 'i', long)]
        input: PathBuf,
        /// Output ZIP file path.
        #[arg(short = 'o', long)]
        output: PathBuf,
        /// Filename encoding for ZIP entry names (default: utf-8).
        #[arg(short = 'e', long, value_name = "ENCODING", default_value = "utf-8")]
        encoding: String,
        /// Recursively include nested files and directories.
        #[arg(short = 'r', long)]
        recursive: bool,
        /// Only include paths matching this glob (repeatable).
        #[arg(long = "include", value_name = "GLOB")]
        include: Vec<String>,
        /// Exclude paths matching this glob (repeatable).
        #[arg(long = "exclude", value_name = "GLOB")]
        exclude: Vec<String>,
    },
    /// Unpack a ZIP archive using a specific filename encoding.
    Unpack {
        /// Input ZIP archive path.
        #[arg(short = 'i', long)]
        input: PathBuf,
        /// Output directory for extracted files.
        #[arg(short = 'o', long)]
        output: PathBuf,
        /// Filename encoding used for ZIP entry names (default: utf-8).
        #[arg(short = 'e', long, value_name = "ENCODING", default_value = "utf-8")]
        encoding: String,
        /// Only extract paths matching this glob (repeatable).
        #[arg(long = "include", value_name = "GLOB")]
        include: Vec<String>,
        /// Skip paths matching this glob (repeatable).
        #[arg(long = "exclude", value_name = "GLOB")]
        exclude: Vec<String>,
    },
}
