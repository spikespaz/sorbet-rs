#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos.rs")]
mod platform;

pub use platform::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("fontconfig could not be initialized")]
    FontconfigInit,
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
