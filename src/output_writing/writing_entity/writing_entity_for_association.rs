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

impl CMOFAssociation {
    /// write content to output file,from "CMOFClass" object
    pub fn write_content(&self, wrt: &mut File) -> Result<(), anyhow::Error> {
        // // Only for "Many to Many"
        // let association = content.get_association_relation();
        // if association.ponteration_type == RelationPonderationType::ManyToMany {
        //     if !association.is_self_referencing {
        //         // // Get file
        //         // let (_, mut wrt) = self.get_object_file(entity);
        //         // //
        //         // content.write_content(&mut wrt, &self.pre_calculation)?;
        //     } else {
        //         warn!(
        //             "Need association file implement for \"{}\" because it's referencin itself",
        //             content.model_name
        //         );
        //     }
        // }
        let association = self.get_association_relation()?;
        match association {
            Relation::OneToOneRelation(relation) => self.write_one_to_one(wrt, relation)?,
            Relation::OneToManyRelation(relation) => self.write_one_to_many(wrt, relation)?,
            Relation::ManyToManyRelation(relation) => self.write_many_to_many(wrt, relation)?,
        }
        Ok(())
    }

    fn write_one_to_one(
        &self,
        wrt: &mut File,
        _relation: OneToOneRelation,
    ) -> Result<(), anyhow::Error> {
        let _ = writeln!(
            wrt,
            include_str!("template/entity_main_association_one_to_one.tmpl"),
            full_name = self.full_name,
            raw = format!("{:#?}", self).prefix("// "),
        );
        Ok(())
    }

    fn write_one_to_many(
        &self,
        wrt: &mut File,
        _relation: OneToManyRelation,
    ) -> Result<(), anyhow::Error> {
        let _ = writeln!(
            wrt,
            include_str!("template/entity_main_association_one_to_many.tmpl"),
            full_name = self.full_name,
            raw = format!("{:#?}", self).prefix("// "),
        );
        Ok(())
    }

    fn write_many_to_many(
        &self,
        wrt: &mut File,
        relation: ManyToManyRelation,
    ) -> Result<(), anyhow::Error> {
        // Get relation 1 content
        let relation_1 = relation.get_from().get_type()?.upgrade()?;
        let relation_1_named = match relation_1.clone() {
            EnumCMOF::CMOFAssociation(c) => c.model_name.clone(),
            EnumCMOF::CMOFClass(c) => c.model_name.clone(),
            EnumCMOF::CMOFDataType(c) => c.model_name.clone(),
            EnumCMOF::CMOFEnumeration(c) => c.model_name.clone(),
            EnumCMOF::CMOFPrimitiveType(c) => c.model_name.clone(),
            _ => panic!("jfsrs894g8sg98"),
        };
        let relation_1_table_name = match relation_1.clone() {
            EnumCMOF::CMOFAssociation(c) => c.table_name.clone(),
            EnumCMOF::CMOFClass(c) => c.table_name.clone(),
            EnumCMOF::CMOFDataType(c) => c.table_name.clone(),
            EnumCMOF::CMOFEnumeration(c) => c.table_name.clone(),
            EnumCMOF::CMOFPrimitiveType(c) => c.table_name.clone(),
            _ => panic!("jfsrs894g8sg9ef'z-ursuefs8"),
        };
        let relation_1_column_name_camel = &&relation_1_named;
        // Get relation 2 content
        let relation_2 = relation.get_to().get_type()?.upgrade()?;
        let relation_2_named = match relation_2.clone() {
            EnumCMOF::CMOFAssociation(c) => c.model_name.clone(),
            EnumCMOF::CMOFClass(c) => c.model_name.clone(),
            EnumCMOF::CMOFDataType(c) => c.model_name.clone(),
            EnumCMOF::CMOFEnumeration(c) => c.model_name.clone(),
            EnumCMOF::CMOFPrimitiveType(c) => c.model_name.clone(),
            _ => panic!("jfsrs894g8sg98"),
        };
        let relation_2_table_name = match relation_2.clone() {
            EnumCMOF::CMOFAssociation(c) => c.table_name.clone(),
            EnumCMOF::CMOFClass(c) => c.table_name.clone(),
            EnumCMOF::CMOFDataType(c) => c.table_name.clone(),
            EnumCMOF::CMOFEnumeration(c) => c.table_name.clone(),
            EnumCMOF::CMOFPrimitiveType(c) => c.table_name.clone(),
            _ => panic!("jfsrs894g8sg9efefs8"),
        };
        let relation_2_column_name_camel = &relation_2_named;
        let _ = writeln!(
            wrt,
            include_str!("template/entity_main_association_many_to_many.tmpl"),
            full_name = self.full_name,
            import = self.get_import_content(),
            table_name = self.table_name,
            relation_1_table_name = relation_1_table_name,
            relation_2_table_name = relation_2_table_name,
            relation_1_column_name_snake = relation_1_column_name_camel.to_case(Case::Snake),
            relation_2_column_name_snake = relation_2_column_name_camel.to_case(Case::Snake),
            relation_1_column_name_camel = relation_1_column_name_camel,
            relation_2_column_name_camel = relation_2_column_name_camel,
            raw = format!("{:#?}", self).prefix("// "),
        );
        Ok(())
    }

    /// "import" content for entity_class_main.tmpl
    fn get_import_content(&self) -> String {
        let mut result = String::from("\n");
        result.push_str("use sea_orm::entity::prelude::*;\n");
        result
    }

    /// Provide information about need to export the association
    pub fn need_file(&self, association: Relation) -> Result<bool, anyhow::Error> {
        if association.is_self_referencing()? {
            return Ok(false);
        }

        Ok(true)
    }
}
