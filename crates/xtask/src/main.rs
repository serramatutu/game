use std::path::PathBuf;

use allocator_api2::alloc::Global as GlobalAllocator;
use clap::{Parser, Subcommand};
use engine::resources::sprite_map::ase_to_res;

/// Game build system and task runner helper
#[derive(Subcommand, Debug)]
enum Commands {
    /// Convert Aseprite-exported assets to `.res` files
    AseToRes {
        /// The resource name
        name: String,
    },
}

/// Game build system and task runner helper
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::AseToRes { name } => {
            ase_to_res(
                GlobalAllocator,
                PathBuf::from("resources/obj").as_path(),
                PathBuf::from(name).as_path(),
            )?;
        }
    }

    Ok(())
}
