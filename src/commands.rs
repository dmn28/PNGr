use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use crate::{args::{Cli, Commands}, chunk::Chunk, chunk_type::ChunkType, png::Png, Result};

pub fn run(args: &Cli) -> Result<()> {
    match &args.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file
        } => {
            encode(file_path, chunk_type, message, output_file)?
        }

        Commands::Decode {
            file_path,
            chunk_type
        } => {
            decode(file_path, chunk_type)?
        }

        Commands::Remove {
            file_path,
            chunk_type
        } => {
            remove(file_path, chunk_type)?
        }

        Commands::Print { file_path } => {
            print_chunks(file_path)?
        }
    }

    Ok(())
}

pub fn encode(file_path: &PathBuf, chunk_type: &str, message: &str, output_file: &Option<PathBuf>) -> Result<()> {
    let file = fs::read(file_path)?;

    let mut png = Png::try_from(&file[..])?;

    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

    png.append_chunk(chunk);

    match output_file {
        Some(f) => {
            fs::write(f, png.as_bytes())?;

            println!("New file has been created and message encoded successfully!");
        }
        None => {
            fs::write(file_path, png.as_bytes())?;

            println!("Message encoded successfully!");
        }
    }

    Ok(())
}

pub fn decode(file_path: &PathBuf, chunk_type: &str) -> Result<()> {
    let file = fs::read(file_path)?;

    let png = Png::try_from(&file[..])?;

    match png.chunk_by_type(chunk_type) {
        Some(chunk) => {
            println!("Message: {:?}", chunk.data_as_string()?)
        }
        None => println!("No message hidden in this image with this chunk type")
    }

    Ok(())
}

pub fn remove(file_path: &PathBuf, chunk_type: &str) -> Result<()> {
    let file = fs::read(file_path)?;

    let mut png = Png::try_from(&file[..])?;

    png.remove_first_chunk(chunk_type)?;

    fs::write(file_path, png.as_bytes())?;

    println!("Message has been removed successfully!");

    Ok(())
}

pub fn print_chunks(file_path: &PathBuf) -> Result<()> {
    let file = fs::read(file_path)?;

    let png = Png::try_from(&file[..])?;

    println!("{}", png);

    Ok(())
}