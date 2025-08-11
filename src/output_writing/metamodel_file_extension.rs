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
#![doc = include_str!("../../doc/writing_entity.md")]

use std::collections::BTreeMap;

// Package section
use crate::cmof_loader::*;
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;

// Dependencies section

// // ####################################################################################################
// //
// // ####################################################################################################

#[derive(Debug, Deserialize)]
struct SimpleValue {
    pub key: String,
    pub value: String,
}

// // ####################################################################################################
// //
// // ####################################################################################################

/// Storage content of "metamodel_file_extension/enumeration_default_value.json" file
pub type EnumerationDefaultValues = BTreeMap<String, String>;

/// Storage content of "metamodel_file_extension/primitive_type_conversion.json" file
pub type PrimitiveTypeConversion = BTreeMap<String, String>;

// // ####################################################################################################
// //
// // ####################################################################################################

/// Provide content of "metamodel_file_extension/enumeration_default_value.json" file
pub fn read_enumeration_default_values() -> Result<EnumerationDefaultValues, anyhow::Error> {
    let reader_path = Path::new("metamodel_file_extension/enumeration_default_value.json");
    let reader = reader_path.get_file_content()?;
    let values: Vec<SimpleValue> = serde_json::from_str(&reader)?;
    let values: EnumerationDefaultValues = values
        .iter()
        .map(|x| (x.key.clone(), x.value.clone()))
        .collect();
    trace!("Read Enumeration Default Values : {:#?}", &values);
    Ok(values)
}

// // ####################################################################################################
// //
// // ####################################################################################################

/// Provide content of "metamodel_file_extension/primitive_type_conversion.json" file
pub fn read_primitive_type_conversion() -> Result<BTreeMap<String, String>, anyhow::Error> {
    let reader_path = Path::new("metamodel_file_extension/primitive_type_conversion.json");
    let reader = reader_path.get_file_content()?;
    let values: Vec<SimpleValue> = serde_json::from_str(&reader)?;
    let values: BTreeMap<String, String> = values
        .iter()
        .map(|x| (x.key.clone(), x.value.clone()))
        .collect();
    trace!("Read Prititive Type Conversion : {:#?}", &values);
    Ok(values)
}
