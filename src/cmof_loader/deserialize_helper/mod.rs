/*
Copyright 2023-2024 CHATROUX MARC

This file is part of Imbriqua Structure, a interpreter of BPMN model files (in UML notation) for
Imbriqua Engine project

Imbriqua Structure is free software: you can redistribute it and/or modify it under the terms of
the GNU General Public License as published by the Free Software Foundation, either
version 3 of the License, or (at your option) any later version.

Imbriqua Structure is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
PURPOSE.See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Imbriqua Structure.
If not, see <https://www.gnu.org/licenses/>.
*/

#![warn(dead_code)]
#![warn(missing_docs)]
#![doc = include_str!("mod.md")]

// Mod section
mod default_values;
mod deser_booleans;
mod deser_collections;
mod deser_numbers;
mod deser_object_ref;
mod deser_rc;
mod deser_string;
pub use default_values::*;
pub use deser_booleans::*;
pub use deser_collections::*;
pub use deser_numbers::*;
pub use deser_object_ref::*;
pub use deser_rc::*;
pub use deser_string::*;
