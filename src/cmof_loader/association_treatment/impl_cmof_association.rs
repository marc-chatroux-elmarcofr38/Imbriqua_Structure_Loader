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

impl CMOFAssociation {
    pub fn get_association_relation(&self) -> Result<Relation, anyhow::Error> {
        let object_0 = self.member_end.0.get_object_as_enum();
        let object_0: &Rc<CMOFProperty> = match object_0.as_ref().unwrap() {
            EnumCMOF::CMOFProperty(c) => c,
            _ => {
                panic!("etrtyuio_0")
            }
        };
        let object_1 = self.member_end.1.get_object_as_enum();
        let object_1: &Rc<CMOFProperty> = match object_1.as_ref().unwrap() {
            EnumCMOF::CMOFProperty(c) => c,
            _ => {
                panic!("etrtyuio_1")
            }
        };
        if object_0.upper > infinitable::Finite(1) && object_1.upper > infinitable::Finite(1) {
            let r = ManyToManyRelation::new(object_0.clone(), object_1.clone())?;
            Ok(Relation::ManyToManyRelation(r))
        } else if object_0.upper > infinitable::Finite(1) {
            let r = OneToManyRelation::new(object_0.clone(), object_1.clone())?;
            Ok(Relation::OneToManyRelation(r))
        } else if object_1.upper > infinitable::Finite(1) {
            let r = OneToManyRelation::new(object_1.clone(), object_0.clone())?;
            Ok(Relation::OneToManyRelation(r))
        } else {
            let r = OneToOneRelation::new(object_1.clone(), object_0.clone())?;
            Ok(Relation::OneToOneRelation(r))
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
