use std::path::PathBuf;

use thiserror::Error;

#[cfg(target_os = "linux")]
use fontconfig::Fontconfig;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(target_os = "linux")]
    #[error("fontconfig could not be initialized")]
    FontconfigInit,
}

/// Locate a font on the filesystem by deferring to platform-specific APIs.
pub fn locate_font<F, S>(family: F, style: Option<S>) -> Result<Option<PathBuf>, Error>
where
    F: AsRef<str>,
    S: AsRef<str>,
{
    #[cfg(target_os = "linux")]
    {
        let config = Fontconfig::new().ok_or(Error::FontconfigInit)?;
        Ok(config
            .find(family.as_ref(), style.as_ref().map(S::as_ref))
            .map(|font| font.path))
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("Arial", None ; "test locate Arial")]
    #[test_case("Monospace", None ; "test locate Monospace")]
    #[test_case("Times New Roman", None ; "test locate Times New Roman")]
    #[test_case("Courrier", None ; "test locate Courrier")]
    fn locate_font(family: &str, style: Option<&str>) {
        match super::locate_font(family, style) {
            Ok(Some(path)) => println!("Found family '{family}': {path:?}"),
            _ => panic!("fontconfig failed to initialize or a font wasn't found"),
        }
    }
}
