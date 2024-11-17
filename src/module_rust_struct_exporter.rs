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

#![warn(missing_docs)]
#![doc = include_str!("../doc/doc_template.md")]

// Package section
use crate::module_file_manager::*;

// Dependencies section
use std::{collections::HashMap, fmt::Debug};

/// Implement writing of target struct instance as Rust struct format
pub trait WritingSruct: Debug {
    /// Implement writing of target struct instance as Rust struct format
    fn wrt_struct_level(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

/// Implement writing of target struct validationfunction as Rust format
pub trait WritingValidation: Debug {
    /// Implement writing of target struct instance as Rust struct format
    fn wrt_sub_validation(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }

    /// Implement writing of target struct instance as Rust struct format
    fn wrt_main_validation(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

/// Implement writing of target struct instance as Rust use import format
pub trait WritingUse: Debug {
    /// Implement writing of target struct instance as Rust use import format
    fn wrt_use_level(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

/// Implement writing of target struct instance as Rust struct trait implementation
pub trait WritingTrait: Debug {
    /// Implement writing of target struct instance as Rust struct trait implementation
    fn wrt_trait_level(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}
