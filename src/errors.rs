use thiserror::Error;

#[derive(Error, Debug)]
pub enum PatchError {
    #[error("Patch file does not exist or is not a file")]
    PatchFileDoesNotExist(),

    #[error("Game directory does not exist or is not a directory")]
    GameDirectoryDoesNotExist(),

    #[error("Invalid patch file")]
    InvalidPatchFile(#[from] zip::result::ZipError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}