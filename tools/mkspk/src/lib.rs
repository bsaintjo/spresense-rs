mod clefia;
mod spk;

#[derive(Debug)]
pub enum MkSpkError {
    InvalidElf(String),
    NoLoadablePhdrs,
    NameTooLong,
}

impl std::fmt::Display for MkSpkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MkSpkError::InvalidElf(msg) => write!(f, "invalid ELF: {msg}"),
            MkSpkError::NoLoadablePhdrs => write!(f, "no loadable PT_LOAD segments with p_memsz > 0"),
            MkSpkError::NameTooLong => write!(f, "savename too long (max 63 bytes)"),
        }
    }
}

impl std::error::Error for MkSpkError {}

/// Pack `elf` into a Spresense SPK image.
///
/// - `savename`: install name stored in the package header (max 63 bytes)
/// - `core`: `-c` value passed to `mkspk` (typically 2)
///
/// Returns the raw SPK bytes ready to write to disk.
pub fn pack_spk(elf: &[u8], savename: &str, core: u8) -> Result<Vec<u8>, MkSpkError> {
    spk::build(elf, savename, core)
}
