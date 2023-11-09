mod util;
mod errors;

use std::fs;
use std::path::{PathBuf};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    patch: PathBuf,

    #[arg(short, long)]
    game_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let patch = args.patch;
    let game_dir = args.game_dir;

    if !game_dir.exists() || !game_dir.is_dir() {
        fs::create_dir_all(&game_dir)?;
    }

    util::apply_patch_primitive(patch, game_dir)?;

    Ok(())
}
