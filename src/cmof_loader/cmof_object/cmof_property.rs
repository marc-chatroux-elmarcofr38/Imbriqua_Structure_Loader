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
use std::collections::BTreeMap;

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, XMIIdentity)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Property Object
pub struct CMOFProperty {
    /// xmi:id attribute
    #[serde(deserialize_with = "deser_local_xmi_id")]
    #[serde(rename = "_xmi:id")]
    pub xmi_id: XMIIdLocalReference,
    /// Parent of the XMI object
    #[serde(skip)]
    pub parent: XMIIdReference<EnumWeakCMOF>,
    /// name attribute
    #[serde(rename = "_name")]
    #[serde(deserialize_with = "deser_name")]
    pub name: String,
    /// visibility attribute
    #[serde(rename = "_visibility")]
    #[serde(default = "default_visibility")]
    pub visibility: UMLVisibilityKind,
    /// Optional type attribute (simple type)
    #[serde(deserialize_with = "deser_option_xmi_id")]
    #[serde(default = "default_option")]
    #[serde(rename = "_type")]
    pub simple_type: Option<XMIIdReference<EnumWeakCMOF>>,
    /// Optional type object (complex type)
    #[serde(rename = "type")]
    pub complex_type: Option<EnumType>,
    /// Optional datatype attribute
    #[serde(rename = "_datatype")]
    pub datatype: Option<String>,
    /// Optional lower attribute
    #[serde(rename = "_lower")]
    #[serde(deserialize_with = "deser_lower_bound")]
    #[serde(default = "default_lower")]
    pub lower: i32,
    /// Optional upper attribute
    #[serde(rename = "_upper")]
    #[serde(deserialize_with = "deser_upper_bound")]
    #[serde(default = "default_upper")]
    pub upper: UnlimitedNatural<i32>,
    /// Optional default attribute
    #[serde(rename = "_default")]
    pub default: Option<String>,
    /// isReadOnly attribute
    #[serde(rename = "_isReadOnly")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_read_only: bool,
    /// isComposite attribute
    #[serde(rename = "_isComposite")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_composite: bool,
    /// isUnique attribute
    #[serde(rename = "_isUnique")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_unique: bool,
    /// isOrdered attribute
    #[serde(rename = "_isOrdered")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_ordered: bool,
    /// Optional isAbstract attribute
    #[serde(rename = "_isAbstract")]
    pub is_abstract: Option<String>,
    /// isDerived attribute
    #[serde(rename = "_isDerived")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_derived: bool,
    /// isDerivedUnion attribute
    #[serde(rename = "_isDerivedUnion")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_derived_union: bool,
    /// Optional subsettedProperty attribute
    #[serde(rename = "_subsettedProperty")]
    pub subsetted_property: Option<String>,
    /// Optional owningAssociation attribute
    #[serde(rename = "_owningAssociation")]
    pub owning_association: Option<String>,
    /// Optional association attribute
    #[serde(rename = "_association")]
    #[serde(deserialize_with = "deser_option_xmi_id")]
    #[serde(default = "default_option")]
    pub association: Option<XMIIdReference<EnumWeakCMOF>>,
    /// Optional redefinedProperty object
    #[serde(rename = "redefinedProperty")]
    pub redefined_property_link: Option<EnumRedefinedProperty>,
    /// Optional SubsettedProperty object
    #[serde(rename = "subsettedProperty")]
    pub subsetted_property_link: Option<EnumSubsettedProperty>,
}

// ####################################################################################################
//
// ####################################################################################################

impl SetCMOFTools for CMOFProperty {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Get needed values
        let package_name = dict_setting
            .get("package_name")
            .ok_or(anyhow::format_err!(
                "Dictionnary error in make_post_deserialize"
            ))?
            .clone();
        let parent_name = self.xmi_id.get_object_id();
        // Set local values
        self.xmi_id.set_package_id_if_empty(&package_name);
        if self.simple_type.is_some() {
            self.simple_type
                .as_mut()
                .unwrap()
                .set_package_id_if_empty(&package_name);
        }
        if self.association.is_some() {
            self.association
                .as_mut()
                .unwrap()
                .set_package_id_if_empty(&package_name);
        }
        // Call on child
        if self.complex_type.is_some() {
            self.complex_type
                .as_mut()
                .unwrap()
                .collect_object(dict_setting, dict_object)?;
        }
        if self.redefined_property_link.is_some() {
            self.redefined_property_link
                .as_mut()
                .unwrap()
                .collect_object(dict_setting, dict_object)?;
        }
        if self.subsetted_property_link.is_some() {
            let m = self
                .subsetted_property_link
                .as_mut()
                .unwrap()
                .collect_object(dict_setting, dict_object)?;
        }
        //Return
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Call on child
        if self.complex_type.is_some() {
            self.complex_type
                .as_ref()
                .unwrap()
                .make_post_deserialize(dict_object)?;
        }
        if self.redefined_property_link.is_some() {
            self.redefined_property_link
                .as_ref()
                .unwrap()
                .make_post_deserialize(dict_object)?;
        }
        if self.subsetted_property_link.is_some() {
            self.subsetted_property_link
                .as_ref()
                .unwrap()
                .make_post_deserialize(dict_object)?;
        }
        if self.simple_type.is_some() {
            set_xmi_id_object(self.simple_type.as_ref().unwrap(), dict_object)?;
        }
        if self.association.is_some() {
            set_xmi_id_object(self.association.as_ref().unwrap(), dict_object)?;
        }
        // Self
        set_xmi_id_object(&self.parent, dict_object)?;
        //Return
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl CMOFProperty {
    pub fn get_type(&self) -> Result<EnumWeakCMOF, anyhow::Error> {
        // For field simple
        if self.simple_type.is_some() {
            self.simple_type.as_ref().unwrap().get_object()
        } else {
            match self.complex_type.as_ref().unwrap() {
                EnumType::HRefPrimitiveType(link) => {
                    // Foreign field
                    Ok(EnumWeakCMOF::CMOFPrimitiveType(link.href.get_object()?))
                }
                EnumType::HRefClass(link) => {
                    // Foreign field
                    Ok(EnumWeakCMOF::CMOFClass(link.href.get_object()?))
                }
                EnumType::HRefDataType(link) => {
                    // Foreign field
                    Ok(EnumWeakCMOF::CMOFDataType(link.href.get_object()?))
                }
            }
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn test_01_creation() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            panic!();

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }
}
