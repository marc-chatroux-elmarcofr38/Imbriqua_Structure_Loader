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

use std::ops::Deref;

// Package section
use crate::cmof_loader::*;

// Dependencies section

// ####################################################################################################
//
// ####################################################################################################

impl CMOFAssociation {
    pub fn get_association_relation(&self) -> AssociationRelation {
        let object_0 = self.member_end.0.object.borrow();
        let object_0: &Rc<CMOFProperty> = match object_0.as_ref().unwrap() {
            EnumCMOF::CMOFProperty(c) => c,
            _ => {
                panic!("etrtyuio_0")
            }
        };
        let object_1 = self.member_end.1.object.borrow();
        let object_1: &Rc<CMOFProperty> = match object_1.as_ref().unwrap() {
            EnumCMOF::CMOFProperty(c) => c,
            _ => {
                panic!("etrtyuio_1")
            }
        };
        if object_0.upper > infinitable::Finite(1) && object_1.upper > infinitable::Finite(1) {
            AssociationRelation {
                relation_1: object_0.clone(),
                relation_2: object_1.clone(),
                ponteration_type: RelationPonderationType::ManyToMany,
                is_self_referencing: object_0 == object_1,
            }
        } else if object_0.upper > infinitable::Finite(1) {
            AssociationRelation {
                relation_1: object_0.clone(),
                relation_2: object_1.clone(),
                ponteration_type: RelationPonderationType::OneToMany,
                is_self_referencing: object_0 == object_1,
            }
        } else if object_1.upper > infinitable::Finite(1) {
            AssociationRelation {
                relation_1: object_1.clone(),
                relation_2: object_0.clone(),
                ponteration_type: RelationPonderationType::OneToMany,
                is_self_referencing: object_0 == object_1,
            }
        } else {
            AssociationRelation {
                relation_1: object_0.clone(),
                relation_2: object_1.clone(),
                ponteration_type: RelationPonderationType::OneToOne,
                is_self_referencing: object_0 == object_1,
            }
        }
    }
}
