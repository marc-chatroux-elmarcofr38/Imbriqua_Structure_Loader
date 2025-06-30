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
#![doc = include_str!("../doc/writing_entity.md")]

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;
use crate::loader_cmof_structure::*;
use crate::loader_dependencies_explorer::LoadingPackage as LPckg;
use crate::loader_dependencies_explorer::LoadingPreCalculation as LPreCalc;
use crate::loader_dependencies_explorer::*;
use crate::writing_manager::*;

// Dependencies section

// ####################################################################################################
//
// ########################################## MAIN ####################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Make a module file for each pckg
    pub fn write_mod_object(&mut self) {
        for (label, pckg) in self.get_package_in_order() {
            debug!("Generating sub-mod file for \"{label}\" : START");
            for entity in pckg.get_sorted_iter() {
                match entity {
                    EnumOwnedMember::Class(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(pckg, entity);
                        //
                        content.write_content(&mut wrt, &pckg, &self.pre_calculation);
                    }
                    EnumOwnedMember::DataType(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(pckg, entity);
                        //
                        content.wrt_entity_fields_caller(&mut wrt, &pckg, &self.pre_calculation);
                    }
                    EnumOwnedMember::Enumeration(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(pckg, entity);
                        //
                        content.wrt_entity_fields_caller(&mut wrt, &pckg, &self.pre_calculation);
                    }
                    EnumOwnedMember::PrimitiveType(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(pckg, entity);
                        //
                        content.wrt_entity_fields_caller(&mut wrt, &pckg, &self.pre_calculation);
                    }
                    _ => {}
                }
            }
            info!("Generating sub-mod file for \"{label}\" : Finished");
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl CMOFClass {
    /// write content to output file,from "CMOFClass" object
    fn write_content(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_class_main.tmpl"),
            full_name = self.get_full_name(pckg),
            import = self.get_import_content(pckg, pre_calc),
            table_name = self.get_table_name(pckg),
            fields = self.get_fields_content(pckg, pre_calc),
            relations = self.get_relations_content(pckg, pre_calc),
            raw = format!("{:#?}", self).prefix("// "),
        );
    }

    /// "import" content for entity_class.tmpl
    fn get_import_content(&self, _pckg: &LPckg, _pre_calc: &LPreCalc) -> String {
        let mut result = String::from("\n");
        result.push_str("use crate::*;\n");
        result.push_str("use sea_orm::entity::prelude::*;\n");
        result
    }

    /// "fields" content for entity_class.tmpl
    fn get_fields_content(&self, pckg: &LPckg, pre_calc: &LPreCalc) -> String {
        let mut result = String::from("");

        // For super class
        for class in self.get_all_super() {
            CMOFClass::write_field_super(&class, &mut result, pckg, pre_calc);
        }

        // For complex property

        // For simple property
        for field in self.owned_attribute.iter() {
            match field {
                EnumOwnedAttribute::Property(content) => {
                    CMOFClass::write_field_simple_property(&content, &mut result, pckg, pre_calc);
                }
            }
        }

        result
    }

    /// "relations" content for entity_class.tmpl
    fn get_relations_content(&self, _pckg: &LPckg, _pre_calc: &LPreCalc) -> String {
        let result = String::from("");
        result
    }

    /// Get all "Super" name
    fn get_all_super(&self) -> Vec<String> {
        // As default, empty
        let mut result: Vec<String> = self.super_class.clone();

        // // For super class link
        for link in self.super_class_link.clone() {
            match link {
                EnumSuperClass::Class(content) => {
                    let class = content.href.clone();
                    let class = match class.find(".cmof#") {
                        Some(split_index) => class[split_index..].replace(".cmof#", "").to_string(),
                        None => class,
                    };
                    result.push(class);
                }
            }
        }

        result
    }

    /// Write "Super" __from get_all_super__
    fn write_field_super(class: &String, result: &mut String, _pckg: &LPckg, _pre_calc: &LPreCalc) {
        let comment = class.prefix("SUPER CLASS : ").replace("\n", "");
        let field_name = class
            .to_case(Case::Snake)
            .prefix("super_")
            .replace("\n", "");
        let field_type = String::from("i64");
        result.push_str(
            format!(
                include_str!("../template/entity_field_part_2.tmpl"),
                comment = comment,
                field_name = field_name,
                field_type = field_type,
            )
            .as_str(),
        );
        result.push_str("\n");
    }

    /// Get all simple field
    fn get_all_simple_field(&self) -> Vec<CMOFProperty> {
        // As default, empty
        let mut result: Vec<CMOFProperty> = Vec::new();

        // // // For super class link
        // for link in self.owned_attribute {
        //     match link {
        //         EnumOwnedAttribute::Property(content) => {
        //             let class = content.href.clone();
        //             let class = match class.find(".cmof#") {
        //                 Some(split_index) => class[split_index..].replace(".cmof#", "").to_string(),
        //                 None => class,
        //             };
        //             result.push(class);
        //         }
        //     }
        // }

        result
    }

    fn write_field_simple_property(
        content: &CMOFProperty,
        result: &mut String,
        pckg: &LPckg,
        pre_calc: &LPreCalc,
    ) {
        result.push_str(content.get_field_content(pckg, pre_calc).as_str());
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl WritingModObjectCaller for CMOFDataType {
    fn wrt_entity_fields_caller(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        // Part 1 : Head
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_datatype_part_1.tmpl"),
            full_name = self.get_full_name(pckg),
        );
        if true {
            // for import
            let _ = writeln!(wrt, include_str!("../template/entity_datatype_part_2.tmpl"),);
        }
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_datatype_part_3.tmpl"),
            table_name = self.get_table_name(pckg),
        );

        // // Part 2 : Fields
        for field in self.owned_attribute.iter() {
            match field {
                EnumOwnedAttribute::Property(content) => {
                    content.wrt_property(wrt, pckg, pre_calc);
                }
            }
        }

        // Part 3 : End
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_datatype_part_4.tmpl"),
            raw = format!("{:#?}", self).prefix("// "),
        );
    }
}

impl WritingModObjectCaller for CMOFEnumeration {
    fn wrt_entity_fields_caller(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        // Part 1 : Head
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_enumeration_part_1.tmpl"),
            full_name = self.get_full_name(pckg),
            model_name = self.get_model_name(),
        );

        // // Part 2 : EnumerationLiteral
        for field in self.owned_attribute.iter() {
            match field {
                EnumOwnedLiteral::EnumerationLiteral(content) => {
                    content.wrt_enumeration_literal(wrt, &self, pre_calc);
                }
            }
        }

        // Part 3 : End
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_enumeration_part_4.tmpl"),
            raw = format!("{:#?}", self).prefix("// "),
        );
    }
}

impl WritingModObjectCaller for CMOFPrimitiveType {
    fn wrt_entity_fields_caller(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        // Part 1 : Head
        let object_type = self.name.as_str();
        if pre_calc
            .primitive_type_conversion
            .get(object_type)
            .is_some()
        {
            let content = pre_calc.primitive_type_conversion.get(object_type).unwrap();
            let _ = writeln!(
                wrt,
                include_str!("../template/entity_primitive_type_part_1.tmpl"),
                full_name = self.get_full_name(pckg),
                model_name = self.get_model_name(),
                standard_object = content,
            );
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl CMOFProperty {
    fn wrt_property(&self, wrt: &mut File, _pckg: &LPckg, pre_calc: &LPreCalc) {
        // type
        let name = &self.name.to_case(Case::Snake);

        if false {
            // if need of #[sea_orm ....]
            let _ = writeln!(
                wrt,
                include_str!("../template/entity_field_part_1.tmpl"),
                field_head = "",
            );
        };

        // // Macro line
        // let mut macro_line = String::new();
        // // start of macro
        // macro_line.push_str("    #[builder(");
        // // setter section
        // macro_line.push_str("setter(into");
        // macro_line.push_str(if self.is_option() {
        //     ", strip_option"
        // } else {
        //     ""
        // });
        // macro_line.push_str(")");

        // if self.is_option() && self.default.is_none() {
        //     macro_line.push_str(", default");
        // }

        // if self.default.is_some() {
        //     macro_line.push_str(", default = \"");
        //     if self.is_option() {
        //         macro_line.push_str("Some(");
        //     }
        //     match self.get_type().as_str() {
        //         "Boolean" => macro_line.push_str(self.default.as_ref().unwrap()),
        //         "Integer" => macro_line.push_str(self.default.as_ref().unwrap()),
        //         "Real" => {
        //             let mut value = self.default.as_ref().unwrap().clone();
        //             value.push_str(if !value.contains('.') { ".0" } else { "" });
        //             macro_line.push_str(value.as_str());
        //         }
        //         "String" => {
        //             let content = String::from("String::from(\\\"")
        //                 + self.default.as_ref().unwrap().as_str()
        //                 + "\\\")";
        //             macro_line.push_str(content.as_str());
        //         }
        //         "dc::Boolean" => macro_line.push_str(self.default.as_ref().unwrap()),
        //         "dc::Integer" => macro_line.push_str(self.default.as_ref().unwrap()),
        //         "dc::Real" => macro_line.push_str(self.default.as_ref().unwrap()),
        //         "dc::String" => {
        //             let content = String::from("String::from(\\\"")
        //                 + self.default.as_ref().unwrap().as_str()
        //                 + "\\\")";
        //             macro_line.push_str(content.as_str());
        //         }
        //         _ => {
        //             let content = self.get_type()
        //                 + "::"
        //                 + self
        //                     .default
        //                     .as_ref()
        //                     .unwrap()
        //                     .to_case(Case::UpperCamel)
        //                     .as_str();
        //             macro_line.push_str(content.as_str());
        //         }
        //     }
        //     if self.is_option() {
        //         macro_line.push_str(")");
        //     }
        //     macro_line.push_str("\"")
        // }
        // // end of macro
        // macro_line.push_str(")]");

        // let _ = writeln!(wrt, "{}", macro_line);

        // main line
        // todo!("add conditionnal treatment for primitive property and link property");
        // let object_type = self.get_type();
        // let object_type = object_type.as_str();
        // if PRIMITIVE_TYPE_LINK.get(object_type).is_some() {
        //     let content = PRIMITIVE_TYPE_LINK.get(object_type).unwrap();
        //     let _ = writeln!(
        //         wrt,
        //         "    {a} {name}: {b}{c}{d}{content}{e}{f}{g},",
        //         name = name,
        //         content = content,
        //         a = if self.is_public() { "pub" } else { "" },
        //         b = if self.is_option() { "Option<" } else { "" },
        //         c = if self.is_vec() { "Vec<" } else { "" },
        //         d = if self.is_lifetime_dpt() { "" } else { "" },
        //         // d = if self.is_lifetime_dpt() { "&'a " } else { "" },
        //         e = if self.is_lifetime_dpt() { "" } else { "" },
        //         // e = if self.is_lifetime_dpt() { "<'a>" } else { "" },
        //         f = if self.is_vec() { ">" } else { "" },
        //         g = if self.is_option() { ">" } else { "" }
        //     );
        // } else {
        //     info!("{}", object_type);
        // };
        if self.is_field() {
            let field_name = name;
            let field_type = self.get_type(&pre_calc);
            let _ = writeln!(
                wrt,
                include_str!("../template/entity_field_part_2.tmpl"),
                comment = self.xmi_id,
                field_name = field_name,
                field_type = field_type,
            );
        }
    }

    fn get_field_content(&self, _pckg: &LPckg, pre_calc: &LPreCalc) -> String {
        let mut result = String::from("");

        // type
        let name = &self.name.to_case(Case::Snake);

        if self.is_field() {
            let comment = self.xmi_id.prefix("SIMPLE FIELD : ").replace("\n", "");
            let field_name = name;
            let field_type = self.get_type(&pre_calc);
            result.push_str(
                format!(
                    include_str!("../template/entity_field_part_2.tmpl"),
                    comment = comment,
                    field_name = field_name,
                    field_type = field_type,
                )
                .as_str(),
            );
            result.push_str("\n");
        };

        result
    }
}

impl CMOFEnumerationLiteral {
    fn wrt_enumeration_literal(
        &self,
        wrt: &mut File,
        enumeration: &CMOFEnumeration,
        pre_calc: &LPreCalc,
    ) {
        // For default
        if pre_calc
            .enumeration_default_value
            .contains_key(&enumeration.name)
        {
            let value_1 = pre_calc
                .enumeration_default_value
                .get(&enumeration.name)
                .unwrap();
            let value_2 = &self.name.to_case(Case::UpperCamel);
            if value_1 == value_2 {
                //
                let _ = writeln!(
                    wrt,
                    include_str!("../template/entity_enumeration_part_2.tmpl"),
                );
            }
        } else {
            warn!("No enuneration default value for {}", enumeration.name)
        };

        // Value
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_enumeration_part_3.tmpl"),
            enumeration_value_snake = self.name,
            enumeration_value_camel = self.name.to_case(Case::UpperCamel),
        );
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

impl CMOFProperty {
    /// If is Foreign field or simple field
    fn is_field(&self) -> bool {
        // upper : 1 or 0
        self.upper <= infinitable::Finite(1)
    }

    /// If need to use option
    fn is_option(&self) -> bool {
        self.lower == 0
    }

    ///
    fn _is_foreign(&self, _pre_calc: &LPreCalc) -> bool {
        if !self.is_field() {
            return false;
        }

        if self.simple_type.is_some() {
            !self.association.is_none()
        } else {
            match self.complex_type.as_ref().unwrap() {
                EnumType::PrimitiveTypeLink(_) => false,
                _ => true,
            }
        }
    }

    ///
    fn get_type(&self, pre_calc: &LPreCalc) -> String {
        let mut result = String::new();

        if !self.is_field() {
            return result;
        }

        // OPTION
        result.push_str(if self.is_option() { "Option<" } else { "" });

        // For field simple

        let content = if self.simple_type.is_some() {
            if self.association.is_none() {
                // Simple field
                self.simple_type.as_ref().unwrap().as_str()
            } else {
                // Foreign field
                "i32"
            }
        } else {
            match self.complex_type.as_ref().unwrap() {
                EnumType::PrimitiveTypeLink(link) => {
                    // Simple field
                    let key = link.href.clone();
                    let key = match key.find(".cmof#") {
                        Some(split_index) => key[split_index..].replace(".cmof#", "").to_string(),
                        None => key,
                    };

                    if pre_calc.primitive_type_conversion.get(&key).is_some() {
                        pre_calc.primitive_type_conversion.get(&key).unwrap()
                    } else {
                        info!("Error : unknow PRIMITIVE TYPE{}", key);
                        "i32"
                    }
                }
                EnumType::ClassLink(_) => {
                    // Foreign field
                    "i32"
                }
                EnumType::DataTypeLink(_) => {
                    // Foreign field
                    "i32"
                }
            }
        };
        result.push_str(content);

        // OPTION
        result.push_str(if self.is_option() { ">" } else { "" });

        result
    }
}
