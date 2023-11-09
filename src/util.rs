use std::fs::File;
use std::path::PathBuf;
use crate::errors::PatchError;

pub(crate) fn apply_patch_primitive(patch: PathBuf, game_dir: PathBuf) -> Result<(), PatchError>{
    if !patch.exists() || !patch.is_file() {
        return Err(PatchError::PatchFileDoesNotExist());
    }

    if !game_dir.exists() || !game_dir.is_dir() {
        return Err(PatchError::GameDirectoryDoesNotExist());
    }

    let patch_file = File::open(patch)?;
    let mut archive = zip::ZipArchive::new(patch_file)?;
    archive.extract(&game_dir)?;

    Ok(())
}