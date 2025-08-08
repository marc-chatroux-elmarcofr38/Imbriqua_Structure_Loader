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

use std::cell::RefCell;

// Package section
use crate::cmof_loader::*;

// Dependencies section

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Class Object
pub struct CMOFClass {
    /// xmi:id attribute
    #[serde(deserialize_with = "deser_local_xmi_id")]
    #[serde(rename = "_xmi:id")]
    pub xmi_id: XMIIdLocalReference,
    /// Casing formating of "name" as technical_name
    #[serde(skip)]
    pub parent: XMIIdReference<EnumWeakCMOF>,
    /// name attribute
    #[serde(rename = "_name")]
    name: String,
    /// isAbstract attribute
    #[serde(rename = "_isAbstract")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_abstract: bool,
    /// Optional superClass attribute (simple superClass)
    #[serde(rename = "_superClass")]
    #[serde(deserialize_with = "deser_spaced_xmi_id")]
    #[serde(default = "default_empty_vec")]
    pub super_class: Vec<XMIIdReference<EnumWeakCMOF>>,
    /// Optional superClass object (complex superClass)
    #[serde(rename = "superClass")]
    #[serde(deserialize_with = "deser_superclass_object_xmi_id")]
    #[serde(default = "default_empty_vec")]
    pub super_class_link: Vec<XMIIdReference<EnumWeakCMOF>>,
    /// Optional ownedAttribute object array
    #[serde(rename = "ownedAttribute")]
    #[serde(deserialize_with = "deser_btreemap_using_name_as_key")]
    #[serde(default = "default_empty_btreemap")]
    pub owned_attribute: BTreeMap<String, EnumOwnedAttribute>,
    /// Optional ownedRule object
    #[serde(rename = "ownedRule")]
    #[serde(deserialize_with = "deser_btreemap_using_name_as_key")]
    #[serde(default = "default_empty_btreemap")]
    pub owned_rule: BTreeMap<String, EnumOwnedRule>,
    /// Casing formating of "name" as technical_name
    #[serde(skip)]
    pub technical_name: String,
    /// Casing formating of "name" as table_name
    #[serde(skip)]
    pub table_name: String,
    /// Casing formating of "name" as model_name
    #[serde(skip)]
    pub model_name: String,
    /// Casing formating of "name" as full_name
    #[serde(skip)]
    pub full_name: String,
    /// Casing formating of 'Super' + "name" as model_case
    #[serde(skip)]
    pub super_model_name: String,
    /// Casing formating of "name" as model_case
    #[serde(skip)]
    pub super_field_name: String,
    /// List of class that have Self as SuperLink
    #[serde(skip)]
    pub reverse_super: RefCell<Vec<Weak<CMOFClass>>>,
    ///
    #[serde(skip)]
    pub relation: RefCell<BTreeMap<String, Relation>>,
}

// ####################################################################################################
//
// ####################################################################################################

impl PartialEq for CMOFClass {
    fn eq(&self, other: &Self) -> bool {
        self.xmi_id == other.xmi_id
    }
}

impl Eq for CMOFClass {}

impl PartialOrd for CMOFClass {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CMOFClass {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.xmi_id.cmp(&other.xmi_id)
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetCMOFTools for CMOFClass {
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
        let package_name_snake_case = package_name.to_case(Case::Snake);
        let class_upper_case = self.name.to_case(Case::UpperCamel);
        let class_snake_case = self.name.to_case(Case::Snake);
        let parent_name = self.xmi_id.get_object_id();
        // Set local values
        self.xmi_id.set_package_id_if_empty(&package_name);
        self.technical_name = format!("{}.cmof#{}", package_name, self.name);
        self.table_name = format!("{}_{}", package_name_snake_case, class_snake_case);
        self.model_name = format!("{}", class_upper_case);
        self.full_name = format!("{}_class_{}", package_name_snake_case, class_snake_case);
        self.super_model_name = format!("Super{}", class_upper_case);
        self.super_field_name = format!("super_{}", class_snake_case);
        // Merge super_class and super_class_link
        // Call on child
        dict_setting.insert(String::from("parent_name"), self.xmi_id.get_object_id());
        for p in &mut self.super_class {
            p.set_package_id_if_empty(&package_name);
        }
        for p in &mut self.super_class_link {
            p.set_package_id_if_empty(&package_name);
        }
        for (_, p) in &mut self.owned_attribute {
            match p {
                EnumOwnedAttribute::Property(c) => {
                    let m = Rc::get_mut(c).unwrap();
                    m.parent.set_package_id_if_empty(&package_name);
                    m.parent.set_object_id(&parent_name);
                    m.collect_object(dict_setting, dict_object)?;
                    dict_object.insert(c.get_xmi_id_field()?, EnumCMOF::CMOFProperty(c.clone()));
                }
            }
        }
        for (_, p) in &mut self.owned_rule {
            match p {
                EnumOwnedRule::Constraint(c) => {
                    let m = Rc::get_mut(c).unwrap();
                    m.parent.set_package_id_if_empty(&package_name);
                    m.parent.set_object_id(&parent_name);
                    m.collect_object(dict_setting, dict_object)?;
                    dict_object.insert(c.get_xmi_id_field()?, EnumCMOF::CMOFConstraint(c.clone()));
                }
            }
        }
        //Return
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Call on child
        for (_, p) in &self.owned_attribute {
            match p {
                EnumOwnedAttribute::Property(c) => c.make_post_deserialize(dict_object)?,
            }
        }
        for (_, p) in &self.owned_rule {
            match p {
                EnumOwnedRule::Constraint(c) => c.make_post_deserialize(dict_object)?,
            }
        }
        for p in &self.super_class {
            p.set_xmi_id_object(dict_object)?;
        }
        for p in &self.super_class_link {
            p.set_xmi_id_object(dict_object)?;
        }
        // Self
        self.parent.set_xmi_id_object(dict_object)?;
        //Return
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl GetXMIId for CMOFClass {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
        self.xmi_id.label()
    }
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
        Ok(self.xmi_id.get_object_id())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl CMOFClass {
    pub fn generate_reverse_super_class(
        &self,
        dict_object: &BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        for (_, object) in dict_object {
            match object {
                EnumCMOF::CMOFClass(class) => {
                    for (_, super_class_reference) in class.get_super_class()? {
                        let super_class = super_class_reference.get_object_as_class()?;
                        if self.get_xmi_id_field()? == super_class.get_xmi_id_field()? {
                            self.reverse_super.borrow_mut().push(Rc::downgrade(class));
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn generate_relation(
        &self,
        dict_object: &BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        for (_, owned) in &self.owned_attribute {
            let EnumOwnedAttribute::Property(property) = owned;
            if property.association.is_none() {
                continue;
            }
            let association: &Rc<CMOFAssociation> = &property
                .association
                .as_ref()
                .unwrap()
                .get_object_as_association()?
                .clone();
            let object_1: Rc<CMOFProperty> = association.member_end.0.get_object_as_property()?;
            let object_2: Rc<CMOFProperty> = association.member_end.0.get_object_as_property()?;
            let value: Relation = if object_1.upper > infinitable::Finite(1)
                && object_2.upper > infinitable::Finite(1)
            {
                Relation::ManyToManyRelation(ManyToManyRelation::new(
                    object_1.clone(),
                    object_2.clone(),
                )?)
            } else if object_1.upper > infinitable::Finite(1) {
                Relation::OneToManyRelation(OneToManyRelation::new(
                    object_1.clone(),
                    object_2.clone(),
                )?)
            } else if object_2.upper > infinitable::Finite(1) {
                Relation::OneToManyRelation(OneToManyRelation::new(
                    object_2.clone(),
                    object_1.clone(),
                )?)
            } else {
                Relation::OneToOneRelation(OneToOneRelation::new(
                    object_1.clone(),
                    object_2.clone(),
                )?)
            };
            let key = association.get_xmi_id_field()?;
            self.relation.borrow_mut().insert(key, value);
        }
        Ok(())
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
