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

// ####################################################################################################
//
// ####################################################################################################

impl CMOFClass {
    pub fn get_super_class(
        &self,
    ) -> Result<BTreeMap<String, &XMIIdReference<EnumWeakCMOF>>, anyhow::Error> {
        let mut r: BTreeMap<String, &XMIIdReference<EnumWeakCMOF>> = BTreeMap::new();
        for v in &self.super_class {
            r.insert(v.label()?, v);
        }
        for v in &self.super_class_link {
            r.insert(v.label()?, v);
        }
        Ok(r)
    }
    pub fn get_reverse_super_class(&self) -> Result<Vec<Rc<CMOFClass>>, anyhow::Error> {
        let mut result = Vec::new();
        for obj in &self.reverse_super.borrow().to_vec() {
            let a = obj.upgrade().clone().unwrap();
            result.push(a.clone());
        }
        Ok(result)
    }

    /// Get all direct One To One relation of the class
    pub fn get_all_direct_one_to_one(
        &self,
    ) -> Result<BTreeMap<String, OneToOneRelation>, anyhow::Error> {
        let mut result: BTreeMap<String, OneToOneRelation> = BTreeMap::new();

        // for (k, v) in self
        //     .owned_attribute
        //     .iter()
        //     // Filter only the Property on the Self CMOF Class
        //     .filter(|&(_, v)| matches!(&v, EnumOwnedAttribute::Property(_)))
        //     // Removing enumeration
        //     .map(|(k, EnumOwnedAttribute::Property(v))| (k, v))
        //     // Filter only Property with Some association field
        //     .filter(|&(_, j)| j.association.is_some())
        //     // Catch XMI object of the association field
        //     .map(|(k, w)| (k, &w.association.as_ref().unwrap().object))
        //     .map(|(k, w)| (k, w.borrow()))
        //     .map(|(k, w)| (k, w.as_ref().unwrap().clone()))
        // // // Filter only association (no filter, just check)
        // // .filter(|(_, v)| matches!(v, EnumCMOF::CMOFAsso&self.reverse_super.borrow().to_vec()ciation(_)))
        // // // Catch Association object
        // // .map(|EnumCMOF::CMOFAssociation(v)| v)
        // // // Catch AssociationRelation
        // // .map(|v| v.get_association_relation())
        // // // Filter only OneToOne Relation
        // // .filter(|v| v.ponteration_type == RelationPonderationType::OneToOne)
        // // // Filter direct only (self is relation 1)
        // // .filter(|v| v.relation_1.get_xmi_id_field().is_ok() && self.get_xmi_id_field().is_ok())
        // // .filter(|v| {
        // //     v.relation_1.get_xmi_id_field().unwrap() == self.get_xmi_id_field().unwrap()
        // // })
        // {
        //     match v {
        //         EnumCMOF::CMOFAssociation(association) => {
        //             let relation = association.get_association_relation();
        //             if relation.ponteration_type != RelationPonderationType::OneToOne {
        //                 // Nope
        //             } else if relation.relation_1.parent.get_object_as_enum().is_err() {
        //                 // Nope
        //             } else if relation.relation_2.parent.get_object_as_enum().is_err() {
        //                 // Nope&self.reverse_super.borrow().to_vec()
        //             } else {
        //                 let object_1 = relation.relation_1.parent.get_object_as_enum().unwrap();
        //                 let object_2 = relation.relation_1.parent.get_object_as_enum().unwrap();
        //                 match object_1 {
        //                     EnumCMOF::CMOFAssociation(c) => {
        //                         if c.get_xmi_id_field()? == self.get_xmi_id_field()? {
        //                             result.push((k.clone(), relation.clone()));
        //                         }
        //                     }
        //                     EnumCMOF::CMOFClass(c) => {
        //                         if c.get_xmi_id_field()? == self.get_xmi_id_field()? {
        //                             result.push((k.clone(), relation.clone()));
        //                         }
        //                     }
        //                     _ => {}
        //                 }
        //                 match object_2 {
        //                     EnumCMOF::CMOFAssociation(c) => {
        //                         if c.get_xmi_id_field()? == self.get_xmi_id_field()? {
        //                             result.push((k.clone(), relation.reverse()));
        //                         }
        //                     }
        //                     EnumCMOF::CMOFClass(c) => {
        //                         if c.get_xmi_id_field()? == self.get_xmi_id_field()? {
        //                             result.push((k.clone(), relation.reverse()));
        //                         }
        //                     }
        //                     _ => {}
        //                 }
        //             }
        //         }
        //         _ => {}
        //     }
        // }
        for (key, relation) in &self.relation.borrow().to_owned() {
            match relation {
                Relation::OneToOneRelation(content) => {
                    if content.get_from().parent.label()? == self.xmi_id.label()? {
                        result.insert(key.clone(), content.clone());
                    }
                }
                _ => {}
            }
        }
        Ok(result)
    }
}
