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
