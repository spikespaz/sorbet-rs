/*
 * Copyright 2022 Jacob Birkett
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Provides useful functions and types for working with fonts on various platforms.
//!
//! Generally members here should work on every platform (Linux, Windows, and macOS) without headache,
//! and return a common [`enum@Error`] with variants that may not necessarily need to be handled on every platform.
//! This is done to eliminate the need for callee-code to handle platform specific situations with `#[cfg(...)]`.

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos.rs")]
mod platform;

pub use platform::*;

use thiserror::Error;

/// The common error type returned from methods in this module. This contains variants that can result from any platform;
/// if your application does not work on all three major platforms, you may use `_ => unimplemented!()` when matching.
#[derive(Debug, Error)]
pub enum Error {
    /// On Linux, there is a dependency to `fontconfig`. If it is not found or failed in some other way,
    /// this variant will be used. This is used in the event that [`fontconfig::Fontconfig::new()`] returns [`None`].
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
