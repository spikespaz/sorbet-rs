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

//! This module contains all of the structures for the color-spaces and formats supported by the crate.
//! They are re-exported in the crate-root.

mod hsl;
mod hsla;
mod hsv;
mod hsva;
mod rgb;
mod rgba;

pub use {hsl::*, hsla::*, hsv::*, hsva::*, rgb::*, rgba::*};
