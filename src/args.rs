use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Struct to define the CLI structure
#[derive(Debug, Parser)]
#[command(name = "pngme")]
#[command(
    about = "A CLI tool for encoding, decoding, removing, and printing messages in PNG files."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands that the CLI will support
#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Encode {
        /// The file path to the PNG file
        file_path: PathBuf,

        /// The chunk type
        chunk_type: String,

        /// The message to encode
        message: String,

        /// The optional output file path
        #[arg(short, long)]
        output_file: Option<PathBuf>,
    },
    #[command(arg_required_else_help = true)]
    Decode {
        /// The file path to the PNG file
        file_path: PathBuf,

        /// The chunk type
        chunk_type: String,
    },
    #[command(arg_required_else_help = true)]
    Remove {
        /// The file path to the PNG file
        file_path: PathBuf,

        /// The chunk type
        chunk_type: String,
    },
    #[command(arg_required_else_help = true)]
    Print {
        /// The file path to the PNG file
        file_path: PathBuf,
    },
}
