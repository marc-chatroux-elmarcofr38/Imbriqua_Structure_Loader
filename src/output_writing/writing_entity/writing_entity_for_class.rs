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

// Mod section
use crate::output_writing::writing_entity::*;

// Package section
use crate::cmof_loader::*;
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;

// Dependencies section

// // ####################################################################################################
// //
// // ####################################################################################################

impl CMOFClass {
    /// write content to output file,from "CMOFClass" object
    pub fn write_content(
        &self,
        wrt: &mut File,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<(), anyhow::Error> {
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_main_class.tmpl"),
            full_name = self.full_name,
            import = self.get_import_content()?,
            table_name = self.table_name,
            fields = self.get_fields_content(primitive_type_conversion)?,
            relations = self.get_relation_content()?,
            related = self.get_related_content()?,
            help_doc = self.get_help(primitive_type_conversion)?.prefix("    /// "),
            help_fn = self.get_help(primitive_type_conversion)?,
            raw = format!("{:#?}", self).prefix("// "),
        );
        Ok(())
    }

    /// "import" content for entity_class_main.tmpl
    fn get_import_content(&self) -> Result<String, anyhow::Error> {
        let mut result = String::from("\n");

        // Only for field that use CMOFEnumeration result, i.e. that have simple property without association
        let mut need_import = false;
        for property in self.get_all_simple_field()? {
            if property.simple_type.is_some() {
                need_import = true;
            }
        }
        if need_import {
            result.push_str("use crate::*;\n");
        }

        // For all
        result.push_str("use sea_orm::entity::prelude::*;\n");

        Ok(result)
    }

    /// "fields" content for entity_class_main.tmpl
    fn get_fields_content(
        &self,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<String, anyhow::Error> {
        let mut result = String::from("");

        // For super class
        for (_, class) in self.get_super_class()? {
            CMOFClass::format_field_super(&class, &mut result)?;
        }

        // For complex property
        for field in self.get_all_complex_field()? {
            CMOFClass::format_field_complex_property(
                &field,
                &mut result,
                primitive_type_conversion,
            )?;
        }

        // For simple property
        for field in self.get_all_simple_field()? {
            CMOFClass::format_field_simple_property(
                &field,
                &mut result,
                primitive_type_conversion,
            )?;
        }

        Ok(result)
    }

    /// "relations" content for entity_class_main.tmpl
    fn get_relation_content(&self) -> Result<String, anyhow::Error> {
        let mut result = String::new();

        // For direct "Super"
        for (_, class) in &self.get_super_class()? {
            let object_class = get_object_as_enum(&class)?;
            let object_class = match object_class {
                EnumCMOF::CMOFClass(c) => c,
                _ => panic!("dfghjdfghjdfgh"),
            };
            CMOFClass::format_relation_super_from_one(&self, &object_class, &mut result)?;
        }

        // For reverse "Super"
        for super_class in self.get_reverse_super_class()? {
            CMOFClass::format_relation_super_to_one(&self, &super_class, &mut result)?;
        }

        // // For "One to One"
        // for (association_name, association) in &self.get_all_direct_one_to_one(pre_calc) {
        //     CMOFClass::format_relation_from_one(
        //         association,
        //         association_name,
        //         &mut result,
        //         pckg,
        //         pre_calc,
        //     );
        // }

        // // For "To One"
        // for (association_name, association) in &self.get_all_to_one(pre_calc) {
        //     CMOFClass::format_relation_to_one(
        //         association,
        //         association_name,
        //         &mut result,
        //         pckg,
        //         pre_calc,
        //     );
        // }

        // // For "From Many"
        // for (association_name, association) in &self.get_all_to_one(pre_calc) {
        //     CMOFClass::format_relation_from_many(
        //         association,
        //         association_name,
        //         &mut result,
        //         pckg,
        //         pre_calc,
        //     );
        // }

        Ok(result)
    }

    /// "related" content for entity_class_main.tmpl
    fn get_related_content(&self) -> Result<String, anyhow::Error> {
        let mut result = String::new();

        // For "Super"
        for (_, class) in self.get_super_class()? {
            let object_class = get_object_as_class(class)?;
            CMOFClass::format_related_direct_super(&self, &object_class, &mut result)?;
        }

        // For reverse "Super"
        for super_class in self.get_reverse_super_class()? {
            CMOFClass::format_related_reverse_super(&self, &super_class, &mut result)?;
        }

        // // For "Many To Many"
        // for (association_name, actual_relation, other_relation) in
        //     &self.get_all_many_to_many(pre_calc)
        // {
        //     if actual_relation.element_type != other_relation.element_type {
        //         CMOFClass::format_related_many_to_many(
        //             association_name,
        //             actual_relation,
        //             other_relation,
        //             &mut result,
        //             pre_calc,
        //         );
        //     } else {
        //         warn!(
        //             "Need \"Many to  Many\" implement for \"{}\" linked to itself",
        //             actual_relation.element_type
        //         );
        //     }
        // }

        Ok(result)
    }

    /// Get all simple field
    fn get_all_simple_field(&self) -> Result<Vec<&CMOFProperty>, anyhow::Error> {
        // As default, empty
        let mut result: Vec<&CMOFProperty> = Vec::new();

        for (_, property) in &self.owned_attribute {
            match property {
                EnumOwnedAttribute::Property(content) => {
                    if content.upper > infinitable::Finite(1) {
                        // Not a field, N-N link
                    } else if content.simple_type.is_some() {
                        if content.association.is_none() {
                            result.push(&content)
                        }
                    } else if matches!(
                        content.complex_type.as_ref().unwrap(),
                        EnumType::HRefPrimitiveType(_)
                    ) {
                        result.push(&content);
                    };
                }
            }
        }

        Ok(result)
    }

    /// Get all complex field
    fn get_all_complex_field(&self) -> Result<Vec<&CMOFProperty>, anyhow::Error> {
        // As default, empty
        let mut result: Vec<&CMOFProperty> = Vec::new();

        for (_, property) in &self.owned_attribute {
            match property {
                EnumOwnedAttribute::Property(content) => {
                    if content.upper > infinitable::Finite(1) {
                        // Not a field, N-N link
                    } else if content.simple_type.is_some() {
                        if content.association.is_some() {
                            result.push(&content)
                        }
                    } else if matches!(
                        content.complex_type.as_ref().unwrap(),
                        EnumType::HRefClass(_)
                    ) {
                        result.push(&content);
                    } else if matches!(
                        content.complex_type.as_ref().unwrap(),
                        EnumType::HRefDataType(_)
                    ) {
                        result.push(&content);
                    };
                }
            }
        }

        Ok(result)
    }

    //     /// Get all Man to Many relation of the class
    //     fn get_all_many_to_many(
    //         &self,
    //         pre_calc: &LPreCalc,
    //     ) -> Vec<(String, ElementRelation, ElementRelation)> {
    //         let mut result: Vec<(String, Rc<CMOFProperty>, Rc<CMOFProperty>)> = Vec::new();

    //         let key = &self.model_name;
    //         for (association_name, association) in self.owned_attribute {
    //             let a = match association {
    //                 EnumOwnedAttribute::Property(c) => c.association.unwrap().object,
    //                 _ => {
    //                     panic!("retryehttzhrzrhzhjryk");
    //                 }
    //             };
    //             if key == &association.relation_1.element_type {
    //                 match association.ponteration_type {
    //                     RelationPonderationType::ManyToMany => {
    //                         result.push((
    //                             association_name.clone(),
    //                             association.relation_1.clone(),
    //                             association.relation_2.clone(),
    //                         ));
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //             if key == &association.relation_2.element_type {
    //                 match association.ponteration_type {
    //                     RelationPonderationType::ManyToMany => {
    //                         result.push((
    //                             association_name.clone(),
    //                             association.relation_2.clone(),
    //                             association.relation_1.clone(),
    //                         ));
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }

    //         result.sort_by(|(a, _, _), (b, _, _)| a.cmp(&b));
    //         result
    //     }

    //     /// Get all reverse One To One relation of the class
    //     fn get_all_reverse_one_to_one(
    //         &self,
    //         pre_calc: &LPreCalc,
    //     ) -> Vec<(String, AssociationRelation)> {
    //         let mut result: Vec<(String, AssociationRelation)> = Vec::new();

    //         let key = &self.model_name;
    //         for (association_name, association) in &pre_calc.association_relation {
    //             if key == &association.relation_2.element_type {
    //                 match association.ponteration_type {
    //                     RelationPonderationType::OneToOne => {
    //                         result.push((association_name.clone(), association.clone()));
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }

    //         result
    //     }

    //     /// Get all direct One To Many relation of the class
    //     fn get_all_direct_one_to_many(
    //         &self,
    //         pre_calc: &LPreCalc,
    //     ) -> Vec<(String, AssociationRelation)> {
    //         let mut result: Vec<(String, AssociationRelation)> = Vec::new();

    //         let key = &self.model_name;
    //         for (association_name, association) in &pre_calc.association_relation {
    //             if key == &association.relation_1.element_type {
    //                 match association.ponteration_type {
    //                     RelationPonderationType::OneToMany => {
    //                         result.push((association_name.clone(), association.clone()));
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }

    //         result
    //     }

    //     // /// Get all reverse One To Many relation of the class
    //     // fn get_all_reverse_one_to_many(
    //     //     &self,
    //     //     pre_calc: &LPreCalc,
    //     // ) -> Vec<(String, AssociationRelation)> {
    //     //     let mut result: Vec<(String, AssociationRelation)> = Vec::new();

    //     //     let key = &self.get_model_name();
    //     //     for (association_name, association) in &pre_calc.association_relation {
    //     //         if key == &association.relation_2.element_type {
    //     //             match association.ponteration_type {
    //     //                 RelationPonderationType::OneToMany => {
    //     //                     result.push((association_name.clone(), association.clone()));
    //     //                 }
    //     //                 _ => {}
    //     //             }
    //     //         }
    //     //     }

    //     //     result
    //     // }

    //     // /// Get all direct Many To Many relation of the class
    //     // fn get_all_direct_many_to_many(
    //     //     &self,
    //     //     pre_calc: &LPreCalc,
    //     // ) -> Vec<(String, AssociationRelation)> {
    //     //     let mut result: Vec<(String, AssociationRelation)> = Vec::new();

    //     //     let key = &self.get_model_name();
    //     //     for (association_name, association) in &pre_calc.association_relation {
    //     //         if key == &association.relation_1.element_type {
    //     //             match association.ponteration_type {
    //     //                 RelationPonderationType::ManyToMany => {
    //     //                     result.push((association_name.clone(), association.clone()));
    //     //                 }
    //     //                 _ => {}
    //     //             }
    //     //         }
    //     //     }

    //     //     result
    //     // }

    //     // /// Get all reverse Many To Many relation of the class
    //     // fn get_all_reverse_many_to_many(
    //     //     &self,
    //     //     pre_calc: &LPreCalc,
    //     // ) -> Vec<(String, AssociationRelation)> {
    //     //     let mut result: Vec<(String, AssociationRelation)> = Vec::new();

    //     //     let key = &self.get_model_name();
    //     //     for (association_name, association) in &pre_calc.association_relation {
    //     //         if key == &association.relation_2.element_type {
    //     //             match association.ponteration_type {
    //     //                 RelationPonderationType::ManyToMany => {
    //     //                     result.push((association_name.clone(), association.clone()));
    //     //                 }
    //     //                 _ => {}
    //     //             }
    //     //         }
    //     //     }

    //     //     result
    //     // }

    /// Format "Super" from __get_all_direct_super__, to write field part
    fn format_field_super(
        class: &XMIIdReference<EnumWeakCMOF>,
        result: &mut String,
    ) -> Result<(), anyhow::Error> {
        let object_class = get_object_as_class(&class)?;
        // Comment
        result.push_str(
            format!(
                "    /// SUPER FIELD : {comment}\n",
                comment = &object_class.super_model_name
            )
            .as_str(),
        );
        // Pub element
        let field_name = &object_class.super_field_name;
        let field_type = String::from("i64");
        result.push_str(
            format!(
                "    pub {field_name}: {field_type},\n",
                field_name = field_name,
                field_type = field_type,
            )
            .as_str(),
        );

        Ok(())
    }

    /// Format "Simple property" from __get_all_simple_field__, to write field part
    fn format_field_simple_property(
        content: &CMOFProperty,
        result: &mut String,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<(), anyhow::Error> {
        // Comment
        result.push_str(
            format!(
                "    /// SIMPLE FIELD : {comment}\n",
                comment = content.xmi_id.label()?
            )
            .as_str(),
        );
        // SEA_ORM element
        if content.default.is_some() {
            result.push_str(
                format!(
                    "    #[sea_orm(default_value = \"{default_value}\")]\n",
                    default_value = content.default.as_ref().unwrap()
                )
                .as_str(),
            );
        }
        // Pub element
        result.push_str(
            format!(
                "    pub {field_name}: {field_type},\n",
                field_name = content.get_field_name(),
                field_type = content.get_field_type(primitive_type_conversion)?,
            )
            .as_str(),
        );
        Ok(())
    }

    /// Format "Complex property" from __get_all_complex_field__, to write field part
    fn format_field_complex_property(
        content: &CMOFProperty,
        result: &mut String,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<(), anyhow::Error> {
        // Comment
        result.push_str(
            format!(
                "    /// COMPLEX FIELD : {comment}\n",
                comment = content.xmi_id.label()?
            )
            .as_str(),
        );
        // SEA_ORM element
        // Pub element
        result.push_str(
            format!(
                "    pub {field_name}: {field_type},\n",
                field_name = &content.name.to_case(Case::Snake),
                field_type = content.get_field_type(primitive_type_conversion)?,
            )
            .as_str(),
        );
        Ok(())
    }

    fn format_relation_super_from_one(
        self_class: &CMOFClass,
        super_class: &Rc<CMOFClass>,
        result: &mut String,
    ) -> Result<(), anyhow::Error> {
        let comment = format!(
            "DIRECT SUPER : ONE {} need ONE {}",
            self_class.model_name, super_class.model_name,
        );
        result.push_str(
            format!(
                include_str!("../template/entity_sub_super_relation.tmpl"),
                table_name = super_class.table_name,
                model_name = super_class.model_name,
                comment = comment,
                foreign_field = super_class.super_model_name,
            )
            .as_str(),
        );
        Ok(())
        // let matched_named = pre_calc.owned_member_type_list.get(key).unwrap();
        // let table_name = &matched_named.table_name;
        // let model_name = &matched_named.model_name;
        // let comment = format!("SUPER : ONE {} need ONE {}", class_model_name, model_name,);
        // result.push_str(
        //     format!(
        //         include_str!("../template/entity_sub_super_relation.tmpl"),
        //         table_name = table_name,
        //         model_name = model_name,
        //         comment = comment,
        //         foreign_field = field_name,
        //     )
        //     .as_str(),
        // );
    }

    fn format_relation_super_to_one(
        self_class: &CMOFClass,
        super_class: &Rc<CMOFClass>,
        result: &mut String,
    ) -> Result<(), anyhow::Error> {
        let comment = format!(
            "REVERSE SUPER : ONE {} need ONE {}",
            super_class.model_name, self_class.model_name,
        );
        result.push_str(
            format!(
                include_str!("../template/entity_sub_relation_to_one.tmpl"),
                table_name = super_class.table_name,
                model_name = super_class.model_name,
                comment = comment,
            )
            .as_str(),
        );
        Ok(())
    }

    /// Format "Super" from __get_all_direct_super__, to write related part
    fn format_related_direct_super(
        self_class: &CMOFClass,
        super_class: &Rc<CMOFClass>,
        result: &mut String,
    ) -> Result<(), anyhow::Error> {
        let comment = format!(
            "DIRECT SUPER : ONE {} need ONE {}",
            self_class.model_name, super_class.model_name,
        );
        result.push_str(
            format!(
                include_str!("../template/entity_sub_super_related.tmpl"),
                table_name = super_class.table_name,
                model_name = super_class.model_name,
                comment = comment,
            )
            .as_str(),
        );
        Ok(())
    }
    //     class_model_name: &String,
    //     super_name: &String,
    //     result: &mut String,
    //     pre_calc: &LPreCalc,
    // ) {
    //     let key = super_name;
    //     if pre_calc.owned_member_type_list.contains_key(key) {
    //         let matched_named = pre_calc.owned_member_type_list.get(key).unwrap();
    //         let table_name = &matched_named.table_name;
    //         let model_name = &matched_named.model_name;
    //         let comment = format!("SUPER : ONE {} need ONE {}", class_model_name, model_name,);
    //         result.push_str(
    //             format!(
    //                 include_str!("../template/entity_sub_super_related.tmpl"),
    //                 table_name = table_name,
    //                 model_name = model_name,
    //                 comment = comment,
    //             )
    //             .as_str(),
    //         );
    //     }
    // }

    /// Format inverse of "Super" from __get_all_reverse_super__, to write related part
    fn format_related_reverse_super(
        self_class: &CMOFClass,
        super_class: &Rc<CMOFClass>,
        result: &mut String,
    ) -> Result<(), anyhow::Error> {
        let comment = format!(
            "REVERSE SUPER : ONE {} need ONE {}",
            super_class.model_name, self_class.model_name,
        );
        result.push_str(
            format!(
                include_str!("../template/entity_sub_super_related.tmpl"),
                table_name = super_class.table_name,
                model_name = super_class.model_name,
                comment = comment,
            )
            .as_str(),
        );
        Ok(())
    }

    //     class_model_name: &String,
    //     super_name: &String,
    //     result: &mut String,
    //     pre_calc: &LPreCalc,
    // ) {
    //     let key = super_name;
    //     if pre_calc.owned_member_type_list.contains_key(key) {
    //         let matched_named = pre_calc.owned_member_type_list.get(key).unwrap();
    //         let table_name = &matched_named.table_name;
    //         let model_name = &matched_named.model_name;
    //         let comment = format!("SUPER : ONE {} need ONE {}", model_name, class_model_name,);
    //         result.push_str(
    //             format!(
    //                 include_str!("../template/entity_sub_super_related.tmpl"),
    //                 table_name = table_name,
    //                 model_name = model_name,
    //                 comment = comment,
    //             )
    //             .as_str(),
    //         );
    //     }
    // }

    //     /// Format inverse of "Super" from __get_all_reverse_super__, to write related part
    //     fn format_related_many_to_many(
    //         association_name: &String,
    //         actual_relation: &ElementRelation,
    //         other_relation: &ElementRelation,
    //         result: &mut String,
    //         pre_calc: &LPreCalc,
    //     ) {
    //         let association_named = pre_calc.owned_member_type_list.get(association_name);
    //         let association_table_name = if association_named.is_some() {
    //             &association_named.unwrap().table_name
    //         } else {
    //             &String::new()
    //         };
    //         let comment = format!(
    //             "ManyToMany : with {} using {}",
    //             other_relation.element_type, association_name,
    //         );
    //         result.push_str(
    //             format!(
    //                 include_str!("../template/entity_sub_related_many_to_many.tmpl"),
    //                 association_table_name = association_table_name,
    //                 other_model = other_relation.element_type,
    //                 actual_model = actual_relation.element_type,
    //                 comment = comment,
    //             )
    //             .as_str(),
    //         );
    //     }

    // Return content for "help_doc" in "entity_main_class"
    fn get_help(
        &self,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<String, anyhow::Error> {
        let mut result = String::new();

        // Add a head
        result.push_str(
            format!(
                "# Help document for \"{}\" ({})\n\n",
                self.model_name, self.full_name
            )
            .as_str(),
        );

        // Common
        result.push_str("## Common fields :\n");
        result.push_str("* __id__ (sea_orm only)\n");
        result.push_str("  * type : __i64__\n");
        result.push_str("\n");

        // Attribute : SIMPLE
        let iter_properties = self.get_all_simple_field()?;
        if iter_properties.len() > 0 {
            result.push_str("## Simple fields :\n");
            for property in iter_properties {
                // Property head
                result.push_str(
                    format!(
                        "* __{}__ (xmi_id : \"{}\")\n",
                        property.get_field_name(),
                        property.xmi_id.label()?
                    )
                    .as_str(),
                );

                // Property content
                result.push_str(
                    format!(
                        "  * type : __{}__\n",
                        property.get_field_type(primitive_type_conversion)?
                    )
                    .as_str(),
                );
                if property.default.is_some() {
                    result.push_str(
                        format!("  * default : \"{}\"\n", property.default.as_ref().unwrap())
                            .as_str(),
                    );
                };
            }
            result.push_str("\n");
        }

        // Attribute : Complex (direct One To One)
        let iter_direct_one_to_one = self.get_all_direct_one_to_one()?;
        if iter_direct_one_to_one.len() > 0 {
            result.push_str("## Direct One To One :\n");
            for (relation_name, relation) in iter_direct_one_to_one {
                let from_model_name = relation.get_from_class()?.model_name.clone();
                let to_model_name = relation.get_to_class()?.model_name.clone();
                // Property head
                result.push_str(
                    format!(
                    "* __{from_model_name}__ (__{from_model_name}Model__) from {relation_name}\n",
                )
                    .as_str(),
                );
                result.push_str(
                format!(
                    "  * one-to-one link : ({}-{}) __{to_model_name}__ need ({}-{}) __{from_model_name}__)\n",
                    relation.get_from().lower,
                    relation.get_from().upper,
                    relation.get_to().lower,
                    relation.get_to().upper,
                )
                .as_str(),
            );
                result.push_str(
                format!(
                    "  * callable using find_also_related(__{from_model_name}Model__) from __{}__\n",
                    self.model_name
                )
                .as_str(),
            );
                result.push_str(
                    format!(
                        "  * saved in __{}__ field as foreing key\n",
                        relation.get_from().name
                    )
                    .as_str(),
                );
            }
            result.push_str("\n");
        }

        // // Attribute : Relation (direct One To Many)
        // let iter_direct_one_to_many = self.get_all_direct_one_to_many(pre_calc);
        // if iter_direct_one_to_many.len() > 0 {
        //     result.push_str("## Relation : One To Many :\n");
        // for (association_name, association) in iter_direct_one_to_many {
        //     // Property head
        //     result.push_str(
        //         format!(
        //             "* __{}__ (__{}Model__) from {}\n",
        //             association.relation_2.element_type,
        //             association.relation_2.element_type,
        //             association_name
        //         )
        //         .as_str(),
        //     );
        //     result.push_str(
        //         format!(
        //             "  * one-to-many link : ({}-{}) __{}__ need ({}-{}) __{}__)\n",
        //             association.relation_2.lower,
        //             association.relation_2.upper,
        //             association.relation_1.element_type,
        //             association.relation_1.lower,
        //             association.relation_1.upper,
        //             association.relation_2.element_type
        //         )
        //         .as_str(),
        //     );
        //     result.push_str(
        //         format!(
        //             "  * callable using find_with_related(__{}Model__) from __{}__\n",
        //             association.relation_2.element_type, self.model_name
        //         )
        //         .as_str(),
        //     );
        //     if association.relation_1.from == RelationSource::FromClass {
        //         result.push_str(
        //             format!(
        //                 "  * named {} in BPMN\n",
        //                 association.relation_2.property_name.to_case(Case::Snake)
        //             )
        //             .as_str(),
        //         );
        //     }
        // }
        // result.push_str("\n");
        // }

        // Attribute : Super (direct)
        let iter_direct_super = self.get_super_class()?;
        if iter_direct_super.len() > 0 {
            result.push_str("## Direct Super :\n");
            for (_, class) in iter_direct_super {
                let direct_super = get_object_as_class(&class)?;
                let field_name = &direct_super.super_field_name;
                // Property head
                result.push_str(
                    format!(
                        "* __{}__ (__{}Model__)\n",
                        direct_super.model_name, direct_super.model_name
                    )
                    .as_str(),
                );
                result.push_str(
                    format!(
                        "  * one-to-one link : one __{}__ need one __{}__)\n",
                        self.model_name, direct_super.model_name
                    )
                    .as_str(),
                );
                result.push_str(
                    format!(
                        "  * callable using find_also_related(__{}Model__) from __{}__\n",
                        direct_super.model_name, self.model_name
                    )
                    .as_str(),
                );
                result.push_str(
                    format!("  * saved in __{}__ field as foreing key\n", field_name).as_str(),
                );
            }
            result.push_str("\n");
        }

        // // Attribute : Complex (Reverse One To One)
        // let iter_reverse_one_to_one = self.get_all_reverse_one_to_one(pre_calc);
        // if iter_reverse_one_to_one.len() > 0 {
        //     result.push_str("## Reverse One To One :\n");
        // for (association_name, association) in iter_reverse_one_to_one {
        //     // Property head
        //     result.push_str(
        //         format!(
        //             "* __{}__ (__{}Model__) from {}\n",
        //             association.relation_1.element_type,
        //             association.relation_1.element_type,
        //             association_name
        //         )
        //         .as_str(),
        //     );
        //     result.push_str(
        //         format!(
        //             "  * one-to-one link : ({}-{}) __{}__ need ({}-{}) __{}__)\n",
        //             association.relation_2.lower,
        //             association.relation_2.upper,
        //             association.relation_1.element_type,
        //             association.relation_1.lower,
        //             association.relation_1.upper,
        //             association.relation_2.element_type
        //         )
        //         .as_str(),
        //     );
        //     result.push_str(
        //         format!(
        //             "  * callable using find_also_related(__{}Model__) from __{}__\n",
        //             self.model_name, association.relation_1.element_type
        //         )
        //         .as_str(),
        //     );
        //     result.push_str(
        //         format!(
        //             "  * saved in __{}__ field as foreing key\n",
        //             association.relation_2.property_name.to_case(Case::Snake)
        //         )
        //         .as_str(),
        //     );
        // }
        // result.push_str("\n");
        // }

        // Attribute : Super (reverse)
        let iter_reverse_super = self.get_reverse_super_class()?;
        if iter_reverse_super.len() > 0 {
            result.push_str("## Reverse Super :\n");
            for reverse_super_class in iter_reverse_super {
                let field_name = &self
                    .model_name
                    .to_case(Case::Snake)
                    .prefix("super_")
                    .replace("\n", "");
                let reverse_super: &String = &reverse_super_class.model_name;
                // Property head
                result.push_str(
                    format!("* __{}__ (__{}Model__)\n", reverse_super, reverse_super).as_str(),
                );
                result.push_str(
                    format!(
                        "  * one-to-one link (reverse) : one __{}__ need one __{}__)\n",
                        reverse_super, self.model_name
                    )
                    .as_str(),
                );
                result.push_str(
                    format!(
                        "  * callable using find_also_related(__{}Model__) from __{}__\n",
                        self.model_name, reverse_super
                    )
                    .as_str(),
                );
                result.push_str(
                    format!(
                        "  * saved in __{}__ field as foreing key in __{}Model__\n",
                        field_name, reverse_super
                    )
                    .as_str(),
                );
            }
            result.push_str("\n");
        }

        Ok(result)
    }

    // /// Format of "Super" from __get_all_direct_super__, to write related part
    // fn format_related_reverse_super(
    //     class_model_name: &String,
    //     super_name: &String,
    //     result: &mut String,
    //     pre_calc: &LPreCalc,
    // ) {
    // }
}
