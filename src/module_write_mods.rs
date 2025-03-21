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
#![doc = include_str!("../doc/module_write_mods.md")]

// Package section
use crate::module_cmof_structure::*;
use crate::module_dependencies_explorer::*;
use crate::module_file_manager::*;
use crate::module_log::*;

// Dependencies section
use lazy_static::lazy_static;
pub use serde_json;
use std::collections::HashMap;
use std::fmt::Debug;

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Get output file
    fn get_output_mod_file(&self, package: &LoadingPackage) -> (String, File) {
        // Calculate folder path
        let mut file_name = self.get_output_folder();
        let filename = package.get_lowercase_name() + ".rs";
        file_name.push(&filename);
        // Create empty file
        let writer = file_name.write_new_file();
        (filename.to_string(), writer)
    }
    /// Make a module file for each package
    pub fn write_mods(&mut self) {
        for (_, (label, package)) in self.get_package_in_order() {
            let (filename, mut writing_file) = self.get_output_mod_file(package);
            // Logs
            debug!("Generating \"{filename}\" from \"{label}\" : START");
            // Write mod head
            package.get_json().wrt_mod_head(&mut writing_file);
            // Write mod structs
            package.get_json().wrt_struct_level(&mut writing_file);
            // Logs
            info!("Generating \"{filename}\" from \"{label}\" : Finished");
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

/// Implement writing of target struct instance as Rust struct trait implementation
pub trait WritingModTrait: Debug {
    /// Implement writing of target struct instance as Rust struct trait implementation
    fn wrt_trait_level(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

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
pub trait WritingModStruct: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : struct element (macro for struct and struct)
    fn wrt_struct_level(&self, writer: &mut File) {
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
// ####################################################################################################
//
// ####################################################################################################

impl WritingModHead for CMOFPackage {
    fn wrt_mod_head(&self, writer: &mut File) {
        // Doc title
        let _ = writeln!(writer, "//! {}", self.get_lowercase_name());
        let _ = writeln!(writer, "use derive_builder::Builder;");

        // Import
        for import in self.package_import.iter() {
            match import {
                EnumPackageImport::PackageImport(content) => {
                    content.wrt_mod_head(writer);
                }
            }
        }
    }
}

impl WritingModHead for CMOFPackageImport {
    fn wrt_mod_head(&self, writer: &mut File) {
        let _ = writeln!(writer, "/// Link from {} (PackageImport)", self.xmi_id);
        match &self.imported_package {
            EnumImportedPackage::ImportedPackage(package) => {
                let content = package.href.clone();
                let content = content.replace(".cmof#_0", "");
                let content = content.to_case(Case::Snake);
                let _ = writeln!(writer, "use crate::{};", content);
            }
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl WritingModStruct for CMOFPackage {
    fn wrt_struct_level(&self, writer: &mut File) {
        for class in self.owned_member.iter() {
            class.wrt_struct_level(writer)
        }
    }
}

impl WritingModStruct for EnumOwnedMember {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumOwnedMember::Association(content) => {
                content.wrt_struct_level(writer);
            }
            EnumOwnedMember::Class(content) => {
                content.wrt_struct_level(writer);
            }
            EnumOwnedMember::DataType(content) => {
                content.wrt_struct_level(writer);
            }
            EnumOwnedMember::Enumeration(content) => {
                content.wrt_struct_level(writer);
            }
            EnumOwnedMember::PrimitiveType(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl WritingModStruct for CMOFAssociation {
    fn wrt_struct_level(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = writeln!(writer, "// struct_level : {}", self.name);
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl WritingModStruct for CMOFClass {
    fn wrt_struct_level(&self, writer: &mut File) {
        // Doc
        self.wrt_doc(writer);
        // Start of Struct
        self.wrt_struct_macro(writer);
        // Start of Struct
        self.wrt_struct_start(writer);
        // OwnedAttribute
        for content in self.owned_attribute.iter() {
            content.wrt_struct_level(writer);
        }
        // End of Struct
        self.wrt_struct_end(writer);

        // Rule validation
        if self.owned_rule.len() > 0 {
            // Start
            self.wrt_validation_start(writer);
            // Load each necessary function
            self.wrt_validation_load_function(writer);
            // Make validation function (using necessary function)
            self.wrt_validation_build(writer);
            // End
            self.wrt_validation_end(writer);
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl WritingModStruct for CMOFDataType {
    fn wrt_struct_level(&self, writer: &mut File) {
        // Doc
        let _ = writeln!(writer);
        let _ = writeln!(
            writer,
            "/// Conversion of {} (DataType : {})",
            self.xmi_id, self.name
        );

        // Start of Struct
        let _ = writeln!(writer, "#[derive(Builder, Debug, Clone)]");

        // Add validation if have constraint
        if self.owned_rule.len() > 0 {
            let _ = writeln!(
                writer,
                "#[builder(build_fn(validate = \"Self::validate\"))]"
            );
        }

        let _ = writeln!(writer, "pub struct {} {{", self.name);

        // OwnedAttribute
        for content in self.owned_attribute.iter() {
            content.wrt_struct_level(writer);
        }

        // End of struct
        let _ = writeln!(writer, "}}");
        let _ = writeln!(writer, "");

        // ownedRule
        if self.owned_rule.len() > 0 {
            // Start
            let _ = writeln!(writer, "impl {}Builder {{", self.name);

            // Sub function
            for content in self.owned_rule.iter() {
                content.wrt_sub_validation(writer);
            }

            // Validation
            let _ = writeln!(writer, "    fn validate(&self) -> Result<(), String> {{");
            for content in self.owned_rule.iter() {
                content.wrt_main_validation(writer);
            }
            let _ = writeln!(writer, "");
            let _ = writeln!(writer, "        return Ok(());");

            // End
            let _ = writeln!(writer, "    }}");
            let _ = writeln!(writer, "}}");
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl WritingModStruct for CMOFEnumeration {
    fn wrt_struct_level(&self, writer: &mut File) {
        // Doc
        let _ = writeln!(writer);
        let _ = writeln!(
            writer,
            "/// Conversion of {} (Enumeration : {})",
            self.xmi_id, self.name
        );

        // Enum
        let _ = writeln!(writer, "#[derive(Debug, Clone)]");
        let _ = writeln!(
            writer,
            "pub enum {} {{",
            self.name.to_case(Case::UpperCamel)
        );
        for content in self.owned_attribute.iter() {
            content.wrt_struct_level(writer);
        }
        let _ = writeln!(writer, "}}");
    }
}

impl WritingModStruct for EnumOwnedLiteral {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumOwnedLiteral::EnumerationLiteral(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

impl WritingModStruct for CMOFEnumerationLiteral {
    fn wrt_struct_level(&self, writer: &mut File) {
        let _ = writeln!(
            writer,
            "    /// '{}' from (id : '{}', name : '{}')",
            self.name.to_case(Case::UpperCamel),
            self.xmi_id,
            self.name
        );
        let _ = writeln!(writer, "    {}, ", self.name.to_case(Case::UpperCamel));
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl WritingModStruct for CMOFPrimitiveType {
    fn wrt_struct_level(&self, writer: &mut File) {
        // Doc
        let _ = writeln!(writer);
        let _ = writeln!(
            writer,
            "/// Conversion of {} (PrimitiveType : {})",
            self.xmi_id, self.name
        );

        // Importing linked struct
        if PRIMITIVE_TYPE_LINK.get(self.name.as_str()).is_some() {
            let content = PRIMITIVE_TYPE_LINK.get(self.name.as_str()).unwrap();
            let _ = writeln!(writer, "pub use {} as {};", content, self.name);
        } else {
            panic!("PANIC : {} isn't in PRIMITIVE_TYPE_LINK", self.name)
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl WritingModStruct for EnumOwnedAttribute {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumOwnedAttribute::Property(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

impl WritingModStruct for EnumOwnedEnd {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumOwnedEnd::Property(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

impl WritingModStruct for EnumRedefinedProperty {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumRedefinedProperty::Property(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

impl WritingModStruct for EnumSubsettedProperty {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumSubsettedProperty::Property(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

impl WritingModStruct for RedefinedProperty {
    fn wrt_struct_level(&self, writer: &mut File) {
        let _ = writeln!(
            writer,
            "// struct_level : {} (RedefinedProperty)",
            self.href
        );
    }
}

impl WritingModStruct for SubsettedProperty {
    fn wrt_struct_level(&self, writer: &mut File) {
        let _ = writeln!(
            writer,
            "// struct_level : {} (SubsettedProperty)",
            self.href
        );
    }
}

impl WritingModStruct for CMOFProperty {
    fn wrt_struct_level(&self, writer: &mut File) {
        // type
        let name = self.name.to_case(Case::Snake);

        // Macro line
        let mut macro_line = String::new();
        // start of macro
        macro_line.push_str("    #[builder(");
        // setter section
        macro_line.push_str("setter(into");
        macro_line.push_str(if self.is_option() {
            ", strip_option"
        } else {
            ""
        });
        macro_line.push_str(")");

        if self.is_option() && self.default.is_none() {
            macro_line.push_str(", default");
        }

        if self.default.is_some() {
            macro_line.push_str(", default = \"");
            if self.is_option() {
                macro_line.push_str("Some(");
            }
            match self.get_type().as_str() {
                "Boolean" => macro_line.push_str(self.default.as_ref().unwrap()),
                "Integer" => macro_line.push_str(self.default.as_ref().unwrap()),
                "Real" => {
                    let mut value = self.default.as_ref().unwrap().clone();
                    value.push_str(if !value.contains('.') { ".0" } else { "" });
                    macro_line.push_str(value.as_str());
                }
                "String" => {
                    let content = String::from("String::from(\\\"")
                        + self.default.as_ref().unwrap().as_str()
                        + "\\\")";
                    macro_line.push_str(content.as_str());
                }
                "dc::Boolean" => macro_line.push_str(self.default.as_ref().unwrap()),
                "dc::Integer" => macro_line.push_str(self.default.as_ref().unwrap()),
                "dc::Real" => macro_line.push_str(self.default.as_ref().unwrap()),
                "dc::String" => {
                    let content = String::from("String::from(\\\"")
                        + self.default.as_ref().unwrap().as_str()
                        + "\\\")";
                    macro_line.push_str(content.as_str());
                }
                _ => {
                    let content = self.get_type()
                        + "::"
                        + self
                            .default
                            .as_ref()
                            .unwrap()
                            .to_case(Case::UpperCamel)
                            .as_str();
                    macro_line.push_str(content.as_str());
                }
            }
            if self.is_option() {
                macro_line.push_str(")");
            }
            macro_line.push_str("\"")
        }
        // end of macro
        macro_line.push_str(")]");

        let _ = writeln!(writer, "{}", macro_line);

        // main line
        let _ = writeln!(
            writer,
            "    {a} {name}: {b}{c}{d}{content}{e}{f}{g},",
            name = name,
            content = self.get_type(),
            a = if self.is_public() { "pub" } else { "" },
            b = if self.is_option() { "Option<" } else { "" },
            c = if self.is_vec() { "Vec<" } else { "" },
            d = if self.is_lifetime_dpt() { "&'a " } else { "" },
            e = if self.is_lifetime_dpt() { "<'a>" } else { "" },
            f = if self.is_vec() { ">" } else { "" },
            g = if self.is_option() { ">" } else { "" }
        );
    }
}

impl WritingModStruct for EnumType {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumType::ClassLink(content) => {
                let _ = writeln!(
                    writer,
                    "    // struct_level : {} (ComplexType)",
                    content.href
                );
            }
            EnumType::PrimitiveTypeLink(content) => {
                let _ = writeln!(
                    writer,
                    "    // struct_level : {} (ComplexType)",
                    content.href
                );
            }
            EnumType::DataTypeLink(content) => {
                let _ = writeln!(
                    writer,
                    "    // struct_level : {} (ComplexType)",
                    content.href
                );
            }
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

lazy_static! {
    static ref OCL_CONSTRANT_FUNCTION: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(
            "planeElement->forAll(oclIsKindOf(Shape) or oclIsKindOf(Edge))",
            "",
        );
        m.insert(
            "size >=  0",
            "        let input = self.size;
        if input.is_some() {
            if input.unwrap().is_some() {
                if !(input.unwrap().unwrap() >= 0.0) {
                    return Err(\"size less that 0\".to_string());
                };
            }
        }",
        );
        m
    };
}

impl WritingModValidation for EnumOwnedRule {
    fn wrt_sub_validation(&self, writer: &mut File) {
        match self {
            EnumOwnedRule::Constraint(content) => {
                content.wrt_sub_validation(writer);
            }
        }
    }

    fn wrt_main_validation(&self, writer: &mut File) {
        match self {
            EnumOwnedRule::Constraint(content) => {
                content.wrt_main_validation(writer);
            }
        }
    }
}

impl WritingModValidation for CMOFConstraint {
    fn wrt_sub_validation(&self, writer: &mut File) {
        let _ = writeln!(
            writer,
            "    // Rule :  {} - {:?}",
            self.name, self.specification
        );

        match &self.specification {
            EnumSpecification::OpaqueExpression(content) => {
                if content.language == String::from("OCL") {
                    // fn creation
                    let _ = writeln!(
                        writer,
                        "    pub fn {a}(self) -> Result<(), String> {{",
                        a = self.name
                    );
                    // content
                    let function_key = content.body.as_str();
                    if OCL_CONSTRANT_FUNCTION.contains_key(function_key) {
                        let _ = writeln!(
                            writer,
                            "{}",
                            OCL_CONSTRANT_FUNCTION.get(function_key).unwrap()
                        );
                    }
                    // end and fn close
                    let _ = writeln!(writer, "        return Ok(());");
                    let _ = writeln!(writer, "    }}");
                    let _ = writeln!(writer, "");
                } else {
                    let _ = writeln!(
                        writer,
                        "// Unknow constraint language : {}",
                        content.language
                    );
                }
            }
        }
    }

    fn wrt_main_validation(&self, writer: &mut File) {
        let _ = writeln!(
            writer,
            "        // Rule :  {} - {:?}",
            self.name, self.specification
        );

        match &self.specification {
            EnumSpecification::OpaqueExpression(content) => {
                if content.language == String::from("OCL") {
                    let _ = writeln!(writer, "        self.{}()?;", self.name);
                } else {
                    let _ = writeln!(
                        writer,
                        "// Unknow constraint language : {}",
                        content.language
                    );
                }
            }
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl CMOFClass {
    /// Check if an attribute need lifetime
    // fn is_attribute_lifetime_dpt(&self) -> bool {
    //     for content in self.owned_attribute.iter() {
    //         if content.is_lifetime_dpt() {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    // /// Check if this super class need lifetime
    // fn is_super_class_lifetime_dpt(&self) -> bool {
    //     if self.super_class.is_some() {
    //         let contents = self.super_class.as_ref().unwrap();
    //         for content in contents.split(' ') {
    //             // let a = "heritage_".to_string() + content.to_case(Case::Snake).as_str();
    //             let b = content;
    //             if is_lifetime_dpt(b) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    // /// Check if this super class link need lifetime
    // fn is_super_class_link_lifetime_dpt(&self) -> bool {
    //     if self.super_class_link.is_some() {
    //         if self.super_class_link.as_ref().unwrap().is_lifetime_dpt() {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    /// Check if this class need lifetime
    pub fn is_lifetime_dpt(&self) -> bool {
        // let bool_1 = self.is_attribute_lifetime_dpt();
        // let bool_2 = self.is_super_class_lifetime_dpt();
        // let bool_3 = self.is_super_class_link_lifetime_dpt();
        // return bool_1 || bool_2 || bool_3;
        return true;
    }
    /// Write raw struct file as doc
    pub fn wrt_doc(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = writeln!(
            writer,
            "/// Conversion of {} (Class : {})",
            self.xmi_id, self.name
        );
        let _ = writeln!(writer, "///");
        let _ = writeln!(writer, "/// ```json");
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("/// "));
        let _ = writeln!(writer, "/// ```");
        let _ = writeln!(writer, "");
    }
    /// Write struct macro
    pub fn wrt_struct_macro(&self, writer: &mut File) {
        let _ = writeln!(writer, "#[derive(Builder, Debug, Clone)]");
        // Add validation if have constraint
        if self.owned_rule.len() > 0 {
            let _ = writeln!(
                writer,
                "#[builder(build_fn(validate = \"Self::validate\"))]"
            );
        }
    }
    /// Write struct start part
    pub fn wrt_struct_start(&self, writer: &mut File) {
        let _ = writeln!(
            writer,
            "pub struct {a}{b} {{",
            a = self.name,
            b = if self.is_lifetime_dpt() { "<'a>" } else { "" }
        );
    }
    /// Write struct end part
    pub fn wrt_struct_end(&self, writer: &mut File) {
        let _ = writeln!(writer, "}}");
        let _ = writeln!(writer, "");
    }
    /// Write struct heritage part
    pub fn wrt_struct_heritage(&self, writer: &mut File) {
        if self.super_class.is_some() {
            let contents = self.super_class.as_ref().unwrap();
            for content in contents.split(' ') {
                let a = "heritage_".to_string() + content.to_case(Case::Snake).as_str();
                let b = content;
                let _ = writeln!(writer, "    pub {a} : {b}, //super_class");
            }
        } else if self.super_class_link.is_some() {
            match self.super_class_link.as_ref().unwrap() {
                EnumSuperClass::Class(content) => match content.href.find('#') {
                    Some(_) => {
                        let result = content.cut_split();
                        let _ = writeln!(
                            writer,
                            "    pub heritage_{a} :{b} {c}::{d}{e}, //super_class_link",
                            a = result.0,
                            b = "",
                            c = result.1,
                            d = result.2,
                            e = ""
                        );
                    }
                    None => {
                        panic!("href without '#' : {}", content.href)
                    }
                },
            }
        }
    }
    /// Write validation start part
    pub fn wrt_validation_start(&self, writer: &mut File) {
        // Start
        let _ = writeln!(
            writer,
            "impl{b} {a}Builder{b} {{",
            a = self.name,
            b = if self.is_lifetime_dpt() { "<'a>" } else { "" }
        );
    }
    /// Write validation end part
    pub fn wrt_validation_load_function(&self, writer: &mut File) {
        for content in self.owned_rule.iter() {
            content.wrt_sub_validation(writer);
        }
    }
    /// Write validation end part
    pub fn wrt_validation_build(&self, writer: &mut File) {
        let _ = writeln!(writer, "    fn validatea(self) -> Result<(), String> {{");
        for content in self.owned_rule.iter() {
            content.wrt_main_validation(writer);
        }
        let _ = writeln!(writer, "");
        let _ = writeln!(writer, "        return Ok(());");
    }
    /// Write validation end part
    pub fn wrt_validation_end(&self, writer: &mut File) {
        let _ = writeln!(writer, "    }}");
        let _ = writeln!(writer, "}}");
    }
}

impl CMOFProperty {
    fn is_public(&self) -> bool {
        self.visibility == EnumVisibilityKind::Public
    }

    fn is_vec(&self) -> bool {
        self.upper > infinitable::Finite(1)
    }

    fn is_option(&self) -> bool {
        self.lower == 0
    }

    fn get_type(&self) -> String {
        if self.simple_type.is_some() {
            let property_type = self.simple_type.as_ref().unwrap();
            property_type.clone()
        } else if self.complex_type.is_some() {
            self.complex_type.as_ref().unwrap().get_type_name()
        } else {
            String::from("None")
        }
    }

    fn is_lifetime_dpt(&self) -> bool {
        is_lifetime_dpt(self.get_type().as_str())
    }
}

impl EnumOwnedAttribute {
    /// Bullshit function : define if a type (represent as string.....) involve to set a structure using reference ("&")
    pub fn is_lifetime_dpt(&self) -> bool {
        match self {
            EnumOwnedAttribute::Property(content) => {
                return content.is_lifetime_dpt();
            }
        }
    }
}

impl EnumSuperClass {
    /// Bullshit function : define if a type (represent as string.....) involve to set a structure using reference ("&")
    pub fn is_lifetime_dpt(&self) -> bool {
        match self {
            EnumSuperClass::Class(content) => {
                return content.is_lifetime_dpt();
            }
        }
    }
}

impl SuperClass {
    /// Ctting href in (Class [SnakeCase], File [SnakeCase], Class)
    pub fn cut_split(&self) -> (String, String, String) {
        let content = self.href.clone();
        let split_index = content.find('#').unwrap();
        let package_file: String = content[..split_index].to_string();
        let package_file: String = package_file.replace(".cmof", "");
        let split_index = split_index + 1;
        let package_class: String = content[split_index..].to_string();

        let a = package_class.to_case(Case::Snake);
        let b = package_file.to_case(Case::Snake);
        let c = package_class;

        let result = (a, b, c);
        return result;
    }

    /// Superclass lifetype type
    pub fn is_lifetime_dpt(&self) -> bool {
        let (_, content_1, content_2) = self.cut_split();
        let name = content_1 + "::" + content_2.as_str();
        return is_lifetime_dpt(name.as_str());
    }
}

impl EnumType {
    /// Name of the "EnumType" object
    pub fn get_type_name(&self) -> String {
        match self {
            EnumType::ClassLink(_) => String::from("i8"),
            EnumType::DataTypeLink(_) => String::from("i8"),
            EnumType::PrimitiveTypeLink(content) => {
                let content = content.href.clone();
                match content.find('#') {
                    Some(split_index) => {
                        let package_file: String = content[..split_index].to_string();
                        let package_file: String = package_file.replace(".cmof", "");
                        let package_file: String = package_file.to_ascii_lowercase();
                        let split_index = split_index + 1;
                        let package_class: String = content[split_index..].to_string();
                        String::from(package_file + "::" + package_class.as_str())
                    }
                    None => {
                        panic!("href without '#' : {}", content)
                    }
                }
            }
        }
    }
}
