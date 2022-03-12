use super::Error;

use std::path::PathBuf;

use fontconfig::Fontconfig;

/// Locate a font on the filesystem by deferring to platform-specific APIs.
pub fn locate_font<F, S>(family: F, style: Option<S>) -> Result<Option<PathBuf>, Error>
where
    F: AsRef<str>,
    S: AsRef<str>,
{
    let config = Fontconfig::new().ok_or(Error::FontconfigInit)?;
    Ok(config
        .find(family.as_ref(), style.as_ref().map(S::as_ref))
        .map(|font| font.path))
}
