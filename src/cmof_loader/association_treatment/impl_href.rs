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

impl XMIIdReference<EnumWeakCMOF> {
    pub fn get_object_as_association(&self) -> Result<Rc<CMOFAssociation>, anyhow::Error> {
        let object_class = self.get_object()?;
        match object_class {
            EnumWeakCMOF::CMOFAssociation(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(content),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            _ => Err(anyhow::format_err!(
                "'get_object_as_association' error, unexpected type ({:?})",
                self
            )),
        }
    }
    pub fn get_object_as_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        let object_class = self.get_object()?;
        match object_class {
            EnumWeakCMOF::CMOFClass(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(content),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            _ => Err(anyhow::format_err!(
                "'get_object_as_class' error, unexpected type ({:?})",
                self
            )),
        }
    }
    pub fn get_object_as_property(&self) -> Result<Rc<CMOFProperty>, anyhow::Error> {
        let object_class = self.get_object()?;
        match object_class {
            EnumWeakCMOF::CMOFProperty(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(content),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            _ => Err(anyhow::format_err!(
                "'get_object_as_property' error, unexpected type ({:?})",
                self
            )),
        }
    }

    /// Get the 'object' of the XMIIdReference, upgrated as EnumCMOF (Rc object)
    pub fn get_object_as_enum(&self) -> Result<EnumCMOF, anyhow::Error> {
        self.get_object()?.upgrade()
    }

    /// Get the weak 'object' of the XMIIdReference, NOT upgrated, as EnumWeakCMOF (Weak object)
    pub fn get_object_as_enum_weak(&self) -> Result<EnumWeakCMOF, anyhow::Error> {
        let object_class = self.get_object()?;
        Ok(object_class)
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
