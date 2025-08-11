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

//! test

// Package section
use crate::cmof_loader::*;

// Dependencies section

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing RedefinedProperty object
pub struct HRefRedefinedProperty {
    /// Link to property of RedefinedProperty
    #[serde(deserialize_with = "deser_xmi_id")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference<Weak<CMOFProperty>>,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing SubsettedProperty object
pub struct HRefSubsettedProperty {
    /// Link to property of SubsettedProperty
    #[serde(deserialize_with = "deser_xmi_id")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference<Weak<CMOFProperty>>,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing SuperClass Tag
pub struct HRefSuperClass {
    /// Link to Class of SuperClass
    #[serde(deserialize_with = "deser_xmi_id")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference<Weak<CMOFClass>>,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing ImportedPackage object
pub struct HRefImportedPackage {
    /// Link of the package
    #[serde(deserialize_with = "deser_xmi_id")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference<Weak<CMOFPackage>>,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Class link
pub struct HRefClass {
    /// Link of the Class type
    #[serde(deserialize_with = "deser_xmi_id")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference<Weak<CMOFClass>>,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Primitive Type link
pub struct HRefPrimitiveType {
    /// Link of the Class type
    #[serde(deserialize_with = "deser_xmi_id")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference<Weak<CMOFPrimitiveType>>,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Data Type link
pub struct HRefDataType {
    /// Link of the Class type
    #[serde(deserialize_with = "deser_xmi_id")]
    #[serde(rename = "_href")]
    pub href: XMIIdReference<Weak<CMOFDataType>>,
}

// ####################################################################################################
//
// ####################################################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmof_loader::tests::check_deser_make_no_error;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn href_redefined_property_01_creation() {
        initialize_log_for_test();

        let value_target = HRefRedefinedProperty {
            href: XMIIdReference::new_local("object_1".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "object_1"}"#, &value_target);

        let value_target = HRefRedefinedProperty {
            href: XMIIdReference::new_global("object_2".to_string(), "package_2".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "package_2.cmof#object_2"}"#, &value_target);
    }

    #[test]
    fn href_subsetted_property_02_creation() {
        initialize_log_for_test();

        let value_target = HRefSubsettedProperty {
            href: XMIIdReference::new_local("object_1".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "object_1"}"#, &value_target);

        let value_target = HRefSubsettedProperty {
            href: XMIIdReference::new_global("object_2".to_string(), "package_2".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "package_2.cmof#object_2"}"#, &value_target);
    }

    #[test]
    fn href_super_class_03_creation() {
        initialize_log_for_test();

        let value_target = HRefSuperClass {
            href: XMIIdReference::new_local("object_1".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "object_1"}"#, &value_target);

        let value_target = HRefSuperClass {
            href: XMIIdReference::new_global("object_2".to_string(), "package_2".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "package_2.cmof#object_2"}"#, &value_target);
    }

    #[test]
    fn href_imported_package_04_creation() {
        initialize_log_for_test();

        let value_target = HRefImportedPackage {
            href: XMIIdReference::new_local("object_1".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "object_1"}"#, &value_target);

        let value_target = HRefImportedPackage {
            href: XMIIdReference::new_global("object_2".to_string(), "package_2".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "package_2.cmof#object_2"}"#, &value_target);
    }

    #[test]
    fn href_class_05_creation() {
        initialize_log_for_test();

        let value_target = HRefClass {
            href: XMIIdReference::new_local("object_1".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "object_1"}"#, &value_target);

        let value_target = HRefClass {
            href: XMIIdReference::new_global("object_2".to_string(), "package_2".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "package_2.cmof#object_2"}"#, &value_target);
    }

    #[test]
    fn href_primitive_type_06_creation() {
        initialize_log_for_test();

        let value_target = HRefPrimitiveType {
            href: XMIIdReference::new_local("object_1".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "object_1"}"#, &value_target);

        let value_target = HRefPrimitiveType {
            href: XMIIdReference::new_global("object_2".to_string(), "package_2".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "package_2.cmof#object_2"}"#, &value_target);
    }

    #[test]
    fn href_data_type_07_creation() {
        initialize_log_for_test();

        let value_target = HRefDataType {
            href: XMIIdReference::new_local("object_1".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "object_1"}"#, &value_target);

        let value_target = HRefDataType {
            href: XMIIdReference::new_global("object_2".to_string(), "package_2".to_string()),
        };
        check_deser_make_no_error(r#"{"_href" : "package_2.cmof#object_2"}"#, &value_target);
    }
}
