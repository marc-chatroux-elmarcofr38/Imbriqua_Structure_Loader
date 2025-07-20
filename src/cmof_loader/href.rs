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
use serde::Deserialize;

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing RedefinedProperty object
pub struct HRefRedefinedProperty {
    /// Link to property of RedefinedProperty
    #[serde(rename = "_href")]
    pub href: String,
}

impl SetCMOFTools for HRefRedefinedProperty {
    fn make_post_deserialize(
        &mut self,
        _dict: &mut std::collections::BTreeMap<String, String>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing SubsettedProperty object
pub struct HRefSubsettedProperty {
    /// Link to property of SubsettedProperty
    #[serde(rename = "_href")]
    pub href: String,
}

impl SetCMOFTools for HRefSubsettedProperty {
    fn make_post_deserialize(
        &mut self,
        _dict: &mut std::collections::BTreeMap<String, String>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing SuperClass Tag
pub struct HRefSuperClass {
    /// Link to Class of SuperClass
    #[serde(rename = "_href")]
    pub href: String,
}

impl SetCMOFTools for HRefSuperClass {
    fn make_post_deserialize(
        &mut self,
        _dict: &mut std::collections::BTreeMap<String, String>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing ImportedPackage object
pub struct HRefImportedPackage {
    /// Link of the package
    #[serde(rename = "_href")]
    pub href: String,
}

impl SetCMOFTools for HRefImportedPackage {
    fn make_post_deserialize(
        &mut self,
        _dict: &mut std::collections::BTreeMap<String, String>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Class link
pub struct HRefClass {
    /// Link of the Class type
    #[serde(rename = "_href")]
    pub href: String,
}

impl SetCMOFTools for HRefClass {
    fn make_post_deserialize(
        &mut self,
        _dict: &mut std::collections::BTreeMap<String, String>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Primitive Type link
pub struct HRefPrimitiveType {
    /// Link of the Class type
    #[serde(rename = "_href")]
    pub href: String,
}

impl SetCMOFTools for HRefPrimitiveType {
    fn make_post_deserialize(
        &mut self,
        _dict: &mut std::collections::BTreeMap<String, String>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Data Type link
pub struct HRefDataType {
    /// Link of the Class type
    #[serde(rename = "_href")]
    pub href: String,
}

impl SetCMOFTools for HRefDataType {
    fn make_post_deserialize(
        &mut self,
        _dict: &mut std::collections::BTreeMap<String, String>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

fn cut_href_and_edit(content: &String) -> (String, String, String) {
    match content.find('#') {
        Some(split_index) => {
            let package: String = content[..split_index]
                .to_string()
                .replace(".cmof", "")
                .to_ascii_lowercase()
                .to_case(Case::Snake);
            let split_index = split_index + 1;
            let class: String = content[split_index..].to_string();
            let result = package.clone() + "::" + class.as_str();
            return (package, class, result);
        }
        None => {
            error!(
                "href attribute without '#' separator : href = \"{}\"",
                content
            );
            panic!(
                "href attribute without '#' separator : href = \"{}\"",
                content
            );
        }
    }
}

impl HRefClass {
    /// Get the package of href
    pub fn get_package(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.0;
    }
    /// Get the class of href
    pub fn get_class(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.1;
    }
    /// Get the class link of href
    pub fn get_rust_call_link(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.2;
    }
}

impl HRefPrimitiveType {
    /// Get the package of href
    pub fn get_package(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.0;
    }
    /// Get the class of href
    pub fn get_class(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.1;
    }
    /// Get the class link of href
    pub fn get_rust_call_link(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.2;
    }
}

impl HRefDataType {
    /// Get the package of href
    pub fn get_package(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.0;
    }
    /// Get the class of href
    pub fn get_class(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.1;
    }
    /// Get the class link of href
    pub fn get_rust_call_link(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.2;
    }
}
