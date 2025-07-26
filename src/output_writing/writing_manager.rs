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
#![doc = include_str!("../../doc/writing_manager.md")]

// Package section
use crate::cmof_loader::*;
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;

// Dependencies section
use infinitable::Infinitable;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt::Debug;

// ####################################################################################################
//
// ###################################### WrittingPath ################################################
//
// ####################################################################################################

/// Trait providing full homogenous path to [`LoadingTracker`]
pub trait WrittingPath {
    /// Get lib.rs file for the LoadingTracker
    ///
    /// Example --> ${output_folder}/src/lib.rs
    fn get_project_lib_file(&self) -> (PathBuf, File);

    /// Get ${package}.rs file for a object of a package
    ///
    /// Example for font object of dc package --> ${output_folder}/src/dc/font.rs
    fn get_object_file(&self, object: &EnumOwnedMember) -> (PathBuf, File);
}

impl WrittingPath for LoadingTracker {
    fn get_project_lib_file(&self) -> (PathBuf, File) {
        // Calculate path
        let mut file_name = self.get_output_folder();
        file_name.push("lib.rs");
        // Create file
        (file_name.clone(), file_name.write_new_file().unwrap())
    }

    fn get_object_file(&self, object: &EnumOwnedMember) -> (PathBuf, File) {
        // Calculate path
        let mut file_name = self.get_output_folder();
        file_name.push(object.get_table_name() + ".rs");
        // Create file
        (file_name.clone(), file_name.write_new_file().unwrap())
    }
}

// ####################################################################################################
//
// ##################################### LoadingTracker ###############################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Build all pre calculing information needed for writting
    pub fn writing_preparation(&mut self) -> Result<(), anyhow::Error> {
        // // owned_member_type_list
        // let mut result: BTreeMap<String, Named> = BTreeMap::new();
        // for (_, package) in self.get_package_in_order() {
        //     for (_, owned_member) in &package.get_json().owned_member {
        //         let key = owned_member.get_xmi_id_field();
        //         let value = Named {
        //             package_name: package.get_json().lowercase_name.clone(),
        //             technical_name: owned_member.get_technical_name(package),
        //             table_name: owned_member.get_table_name(package),
        //             model_name: owned_member.get_model_name(),
        //             full_name: owned_member.get_full_name(package),
        //         };
        //         result.insert(key, value);
        //     }
        // }
        // self.pre_calculation.owned_member_type_list = result.clone();
        // trace!(
        //     "Writing_preparation : owned_member_type_list {:#?}",
        //     self.pre_calculation.owned_member_type_list
        // );

        // // association_relation
        // let mut result: BTreeMap<String, Vec<ElementRelation>> = BTreeMap::new();
        // for (_, package) in self.get_package_in_order() {
        //     for (_, owned_member) in &package.get_json().owned_member {
        //         match owned_member {
        //             EnumOwnedMember::Association(content) => {
        //                 let key = content.model_name.clone();
        //                 for (_, owned_end) in &content.owned_end {
        //                     match owned_end {
        //                         EnumOwnedEnd::Property(property) => {
        //                             let value = ElementRelation {
        //                                 element_type: property.get_type(),
        //                                 property_name: property.name.clone(),
        //                                 lower: property.lower,
        //                                 upper: property.upper,
        //                                 from: RelationSource::FromAssociation,
        //                             };
        //                             if result.contains_key(&key) {
        //                                 let result_vec = result.get_mut(&key).unwrap();
        //                                 result_vec.push(value);
        //                             } else {
        //                                 result.insert(key.clone(), Vec::from([value]));
        //                             };
        //                         }
        //                     }
        //                 }
        //             }
        //             EnumOwnedMember::Class(content) => {
        //                 for (_, owned_attribute) in &content.owned_attribute {
        //                     match owned_attribute {
        //                         EnumOwnedAttribute::Property(property) => {
        //                             if property.association.is_some() {
        //                                 let key = property.association.clone().unwrap();

        //                                 let value = ElementRelation {
        //                                     element_type: property.get_type(),
        //                                     property_name: property.name.clone(),
        //                                     lower: property.lower,
        //                                     upper: property.upper,
        //                                     from: RelationSource::FromClass,
        //                                 };
        //                                 if result.contains_key(&key) {
        //                                     let result_vec = result.get_mut(&key).unwrap();
        //                                     result_vec.push(value);
        //                                 } else {
        //                                     result.insert(key.clone(), Vec::from([value]));
        //                                 };
        //                             };
        //                         }
        //                     }
        //                 }
        //             }
        //             _ => {}
        //         }
        //     }
        // }
        // for (relation_name, association) in result {
        //     if association.len() != 2 {
        //         error!(
        //             "Association multiplicity error : need to be 2, not {num} : {name}",
        //             num = association.len(),
        //             name = relation_name,
        //         );
        //     } else {
        //         let key = relation_name;
        //         let relation_1 = if association[0].upper < association[1].upper {
        //             association[1].clone()
        //         } else {
        //             association[0].clone()
        //         };
        //         let relation_2 = if association[0].upper < association[1].upper {
        //             association[0].clone()
        //         } else {
        //             association[1].clone()
        //         };
        //         let ponteration_type = if relation_2.upper > Infinitable::Finite(1) {
        //             RelationPonderationType::ManyToMany
        //         } else if relation_1.upper > Infinitable::Finite(1) {
        //             RelationPonderationType::OneToMany
        //         } else {
        //             RelationPonderationType::OneToOne
        //         };
        //         let is_self_referencing =
        //             association[0].element_type == association[1].element_type;
        //         let value = AssociationRelation {
        //             relation_1: relation_1,
        //             relation_2: relation_2,
        //             ponteration_type: ponteration_type,
        //             is_self_referencing: is_self_referencing,
        //         };
        //         self.pre_calculation.association_relation.insert(key, value);
        //     }
        // }
        // trace!(
        //     "Writing_preparation : association_relation {:#?}",
        //     self.pre_calculation.association_relation
        // );

        // // association_relation_by_class
        // let mut result: BTreeMap<String, Vec<(String, RankRelation)>> = BTreeMap::new();
        // for (name, association) in &self.pre_calculation.association_relation {
        //     // Relation 1
        //     let member = &association.relation_1;
        //     let key = member.element_type.clone();
        //     let value = (name.clone(), RankRelation::IsOne);

        //     if result.contains_key(&key) {
        //         let result_vec = result.get_mut(&key).unwrap();
        //         result_vec.push(value);
        //     } else {
        //         result.insert(key.clone(), Vec::from([value]));
        //     };
        //     // Relation 2
        //     let member = &association.relation_2;
        //     let key = member.element_type.clone();
        //     let value = (name.clone(), RankRelation::IsSecond);

        //     if result.contains_key(&key) {
        //         let result_vec = result.get_mut(&key).unwrap();
        //         result_vec.push(value);
        //     } else {
        //         result.insert(key.clone(), Vec::from([value]));
        //     };
        // }
        // self.pre_calculation.association_relation_by_class = result.clone();
        // trace!(
        //     "Writing_preparation : association_relation_by_class {:#?}",
        //     self.pre_calculation.association_relation_by_class
        // );

        // // reverse_super_link
        // let mut result: BTreeMap<String, Vec<Rc<CMOFClass>>> = BTreeMap::new();
        // for (_, package) in self.get_package_in_order() {
        //     for (_, owned_member) in &package.get_json().owned_member {
        //         match owned_member {
        //             EnumOwnedMember::Class(content) => {
        //                 // for all element in result
        //                 for (k, super_field) in content.get_super_class() {
        //                     let object_class = super_field.object.borrow();
        //                     let object_class = match object_class.as_ref().unwrap() {
        //                         EnumCMOF::CMOFClass(c) => c,
        //                         _ => panic!("dfghjdfghjdfgh"),
        //                     };
        //                     let key = object_class.get_xmi_id_field();
        //                     let value = content.clone();

        //                     if result.contains_key(&key) {
        //                         let result_vec = result.get_mut(&key).unwrap();
        //                         result_vec.push(value);
        //                     } else {
        //                         result.insert(key.clone(), Vec::from([value]));
        //                     };
        //                 }
        //             }
        //             _ => {}
        //         }
        //     }
        // }
        // self.pre_calculation.reverse_super_link = result;
        // trace!(
        //     "Writing_preparation : reverse_super_link {:#?}",
        //     self.pre_calculation.reverse_super_link
        // );
        Ok(())
    }
}

// ####################################################################################################
//
// ################################### Writting Organiser #############################################
//
// ####################################################################################################

/// Trait for writting __lib.rs__ file from sub-element of [`LoadingPackage`]
pub trait WritingLibFile: Debug {
    /// Writting __lib.rs__ file from sub-element of [`LoadingPackage`]
    fn wrt_lib_file_level(
        &self,
        wrt: &mut File,
        pckg: &LoadingPackage,
        pre_calc: &LoadingPreCalculation,
    ) -> Result<(), anyhow::Error>;
}

/// Trait for writting __${owned_member}.rs__ file from [`EnumOwnedMember`] element of [`LoadingPackage`]
pub trait WritingModObject: Debug {
    /// Writting __${owned_member}.rs__ file from [`EnumOwnedMember`] element of [`LoadingPackage`]
    fn wrt_entity_fields(
        &self,
        wrt: &mut File,
        pckg: &LoadingPackage,
        pre_calc: &LoadingPreCalculation,
    ) -> Result<(), anyhow::Error>;
}

/// Trait for writting __${owned_member}.rs__ struct validation from [`EnumOwnedMember`] element of [`LoadingPackage`]
pub trait WritingModValidation: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : macro adding struct validation
    fn wrt_sub_validation(&self, writer: &mut File) -> Result<(), anyhow::Error>;
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : additionnal validation function for struct validation
    fn wrt_main_validation(&self, writer: &mut File) -> Result<(), anyhow::Error>;
}
