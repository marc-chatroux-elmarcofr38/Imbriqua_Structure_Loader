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
    #[serde(deserialize_with = "deser_href")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference,
}

// impl SetCMOFTools for HRefRedefinedProperty {
//     fn collect_object(
//         &mut self,
//         _dict_setting: &mut BTreeMap<String, String>,
//         _dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         Ok(())
//     }
//     fn make_post_deserialize(
//         &self,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         let k = self.href.label();
//         let r = dict_object.get(&k);
//         if r.is_none() {
//             return Err(anyhow::format_err!(
//                 "Matching error in post_deserialize : \"{}\" not find in dict_object",
//                 k
//             ));
//         } else {
//             self.href.set_load(r.unwrap().clone());
//         }
//         // Return
//         Ok(())
//     }
// }

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing SubsettedProperty object
pub struct HRefSubsettedProperty {
    /// Link to property of SubsettedProperty
    #[serde(deserialize_with = "deser_href")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing SuperClass Tag
pub struct HRefSuperClass {
    /// Link to Class of SuperClass
    #[serde(deserialize_with = "deser_href")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing ImportedPackage object
pub struct HRefImportedPackage {
    /// Link of the package
    #[serde(deserialize_with = "deser_href")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Class link
pub struct HRefClass {
    /// Link of the Class type
    #[serde(deserialize_with = "deser_href")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Primitive Type link
pub struct HRefPrimitiveType {
    /// Link of the Class type
    #[serde(deserialize_with = "deser_href")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Data Type link
pub struct HRefDataType {
    /// Link of the Class type
    #[serde(deserialize_with = "deser_href")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference,
}
