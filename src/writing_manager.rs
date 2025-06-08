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
//! a ecrire

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;
use crate::loader_cmof_structure::*;
use crate::loader_dependencies_explorer::*;

// Dependencies section
pub use serde_json;
use std::fmt::Debug;

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Get output lib.rs file
    pub fn get_output_lib_file(&self) -> (String, File) {
        // Calculate folder path
        let mut file_name = self.get_output_folder();
        let filename = "lib.rs";
        file_name.push(filename);
        // Create empty file
        let writer = file_name.write_new_file();
        (filename.to_string(), writer)
    }
    /// Get output folder for the package --> ex : src/dc
    pub fn get_output_mod_folder(&self, package: &LoadingPackage) -> PathBuf {
        // Calculate folder path
        let mut folder_name = self.get_output_folder();
        let pachage_name = package.get_level_path() + "/";
        folder_name.push(&pachage_name);
        // Create empty file
        let _ = folder_name.create_folder();
        folder_name
    }

    /// Get mod file for the package --> ex : src/dc/mod.rs
    pub fn get_output_mod_file(&self, package: &LoadingPackage) -> (String, File) {
        // Calculate folder path
        let mut file_name = self.get_output_folder();
        let filename = package.get_level_path() + "/mod.rs";
        file_name.push(&filename);
        // Create empty file
        let writer = file_name.write_new_file();
        (filename.to_string(), writer)
    }

    /// Get output file of a object of a package --> ex : src/dc/font.rs
    pub fn get_output_mod_object(package_folder: &PathBuf, object_name: &str) -> (String, File) {
        // Calculate folder path
        let mut file_name = package_folder.clone();
        let filename = String::from(object_name) + ".rs";
        file_name.push(&filename);
        // Create empty file
        let writer = file_name.write_new_file();
        (filename.to_string(), writer)
    }

    /// Build all pre calculing information
    pub fn writing_preparation(&mut self) {
        // Alone classes
        for (_, package) in self.clone().get_package_in_order() {
            for owned_member in package.get_json().owned_member.clone().into_iter() {
                match owned_member {
                    EnumOwnedMember::Class(content) => {
                        let mut is_alone = true;
                        for owned_attribute in content.owned_attribute {
                            match owned_attribute {
                                EnumOwnedAttribute::Property(content_2) => {
                                    if !is_simple_dpt(content_2.name.as_str()) {
                                        is_alone = false;
                                    }
                                }
                            }
                        }
                        if is_alone {
                            self.pre_calculation
                                .class_classification
                                .insert(content.name, ClassClassification::Simple);
                        }
                    }
                    _ => {}
                }
            }
        }
        debug!(
            "Simple Class {:?}",
            self.pre_calculation.class_classification
        );
        // build_class_classification();
    }
}

/// Bullshit function : define if a type (represent as string.....) involve to set a structure using reference ("&")
pub fn is_simple_dpt(input: &str) -> bool {
    match input {
        "Boolean" => false,
        "Integer" => false,
        "Real" => false,
        "String" => false,
        "i8" => false,
        "u8" => false,
        "dc::Boolean" => false,
        "dc::Integer" => false,
        "dc::Real" => false,
        "dc::String" => false,
        _ => true,
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

/// Implement writing of target mod loading head element as Rust
pub trait WritingModHead: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : module file head (import part, "use", etc.)
    fn wrt_mod_head(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

/// Implement writing of target mod loading head element as Rust
pub trait WritingModObjectCall: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : struct element (macro for struct and struct)
    fn wrt_mod_object_call(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

/// Implement writing of target lib loading element as Rust
pub trait WritingLibHead: Debug {
    /// Implement writing of target lib loading element as Rust
    fn wrt_lib_level(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

/// Implement writing of target struct instance as Rust struct trait implementation
pub trait WritingModTrait: Debug {
    /// Implement writing of target struct instance as Rust struct trait implementation
    fn wrt_trait_level(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

/// Implement writing of target mod loading head element as Rust
pub trait WritingCallModObject: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : struct element (macro for struct and struct)
    fn wrt_call_mod_object(&self, folder: &PathBuf, package_name: &str) {
        let _ = folder;
        let _ = package_name;
    }
}

/// Implement writing of target mod loading head element as Rust
pub trait WritingModObject: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : struct element (macro for struct and struct)
    fn wrt_mod_object(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

/// Implement writing of target struct validationfunction as Rust format
pub trait WritingModValidation: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : macro adding struct validation
    fn wrt_sub_validation(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : additionnal validation function for struct validation
    fn wrt_main_validation(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

// ####################################################################################################
//
// ####################################### NamingPath #################################################
//
// ####################################################################################################

/// Provide naming method for CMOF Object : Path
pub trait NamingPath {
    /// Name for object file, add linked import
    fn get_level_path(&self) -> String {
        String::new()
    }
}

impl NamingPath for LoadingPackage {
    fn get_level_path(&self) -> String {
        let mut result: String = String::from("package_");
        result.push_str(&self.get_lowercase_name().to_case(Case::Snake).as_str());
        result
    }
}

impl NamingPath for EnumOwnedMember {
    fn get_level_path(&self) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.get_level_path(),
            EnumOwnedMember::Class(content) => content.get_level_path(),
            EnumOwnedMember::DataType(content) => content.get_level_path(),
            EnumOwnedMember::Enumeration(content) => content.get_level_path(),
            EnumOwnedMember::PrimitiveType(content) => content.get_level_path(),
        }
    }
}

impl NamingPath for ImportedPackage {
    fn get_level_path(&self) -> String {
        let content = self.href.clone();
        let content = content.replace(".cmof#_0", "");
        let mut result = String::from("package_");
        result.push_str(content.to_case(Case::Snake).as_str());
        result
    }
}

impl NamingPath for CMOFAssociation {
    fn get_level_path(&self) -> String {
        let mut result = String::from("link_");
        result.push_str(self.name.to_case(Case::Snake).as_str());
        result
    }
}

impl NamingPath for CMOFClass {
    fn get_level_path(&self) -> String {
        let mut result = String::from("class_");
        result.push_str(self.name.to_case(Case::Snake).as_str());
        result
    }
}

impl NamingPath for CMOFDataType {
    fn get_level_path(&self) -> String {
        let mut result = String::from("datatype_");
        result.push_str(self.name.to_case(Case::Snake).as_str());
        result
    }
}

impl NamingPath for CMOFEnumeration {
    fn get_level_path(&self) -> String {
        let mut result = String::from("enum_");
        result.push_str(self.name.to_case(Case::Snake).as_str());
        result
    }
}

impl NamingPath for CMOFPrimitiveType {
    fn get_level_path(&self) -> String {
        let mut result = String::from("primitivetype_");
        result.push_str(self.name.to_case(Case::Snake).as_str());
        result
    }
}

// ####################################################################################################
//
// ###################################### NamingStruct ################################################
//
// ####################################################################################################

/// Provide naming method for CMOF Object : Struct
pub trait NamingStruct {
    /// Name for object file, add linked import
    fn get_level_struct(&self) -> String {
        String::new()
    }
}

impl NamingStruct for CMOFAssociation {
    fn get_level_struct(&self) -> String {
        let mut result = String::from("");
        result.push_str(self.name.to_case(Case::UpperCamel).as_str());
        result
    }
}

impl NamingStruct for CMOFClass {
    fn get_level_struct(&self) -> String {
        let mut result = String::from("");
        result.push_str(self.name.to_case(Case::UpperCamel).as_str());
        result
    }
}

impl NamingStruct for CMOFDataType {
    fn get_level_struct(&self) -> String {
        let mut result = String::from("");
        result.push_str(self.name.to_case(Case::UpperCamel).as_str());
        result
    }
}

impl NamingStruct for CMOFEnumeration {
    fn get_level_struct(&self) -> String {
        let mut result = String::from("");
        result.push_str(self.name.to_case(Case::UpperCamel).as_str());
        result
    }
}

impl NamingStruct for CMOFPrimitiveType {
    fn get_level_struct(&self) -> String {
        let mut result = String::from("");
        result.push_str(self.name.to_case(Case::UpperCamel).as_str());
        result
    }
}
