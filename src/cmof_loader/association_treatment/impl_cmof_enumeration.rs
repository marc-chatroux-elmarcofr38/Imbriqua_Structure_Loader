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

// Package section
use crate::cmof_loader::*;

// Dependencies section

// ####################################################################################################
//
// ####################################################################################################

impl CMOFEnumeration {
    /// Get all literal
    pub fn get_all_literal(&self) -> Result<Vec<&CMOFEnumerationLiteral>, anyhow::Error> {
        // As default, empty
        let mut result: Vec<&CMOFEnumerationLiteral> = Vec::new();

        for (_, property) in &self.owned_attribute {
            match property {
                EnumOwnedLiteral::EnumerationLiteral(content) => {
                    result.push(&content);
                }
            }
        }

        Ok(result)
    }
}
