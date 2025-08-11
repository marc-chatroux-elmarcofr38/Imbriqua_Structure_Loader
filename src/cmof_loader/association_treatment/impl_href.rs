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

pub fn get_object_as_association(
    object: &XMIIdReference<EnumWeakCMOF>,
) -> Result<Rc<CMOFAssociation>, anyhow::Error> {
    let object_class = object.get_object()?;
    match object_class {
        EnumWeakCMOF::CMOFAssociation(c) => {
            let r = c.upgrade();
            match r {
                Some(content) => Ok(content),
                None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", object)),
            }
        }
        _ => Err(anyhow::format_err!(
            "'get_object_as_association' error, unexpected type ({:?})",
            object
        )),
    }
}
pub fn get_object_as_class(
    object: &XMIIdReference<EnumWeakCMOF>,
) -> Result<Rc<CMOFClass>, anyhow::Error> {
    let object_class = object.get_object()?;
    match object_class {
        EnumWeakCMOF::CMOFClass(c) => {
            let r = c.upgrade();
            match r {
                Some(content) => Ok(content),
                None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", object)),
            }
        }
        _ => Err(anyhow::format_err!(
            "'get_object_as_class' error, unexpected type ({:?})",
            object
        )),
    }
}
pub fn get_object_as_property(
    object: &XMIIdReference<EnumWeakCMOF>,
) -> Result<Rc<CMOFProperty>, anyhow::Error> {
    let object_class = object.get_object()?;
    match object_class {
        EnumWeakCMOF::CMOFProperty(c) => {
            let r = c.upgrade();
            match r {
                Some(content) => Ok(content),
                None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", object)),
            }
        }
        _ => Err(anyhow::format_err!(
            "'get_object_as_property' error, unexpected type ({:?})",
            object
        )),
    }
}

/// Get the 'object' of the XMIIdReference, upgrated as EnumCMOF (Rc object)
pub fn get_object_as_enum(
    object: &XMIIdReference<EnumWeakCMOF>,
) -> Result<EnumCMOF, anyhow::Error> {
    object.get_object()?.upgrade()
}

/// Get the weak 'object' of the XMIIdReference, NOT upgrated, as EnumWeakCMOF (Weak object)
pub fn get_object_as_enum_weak(
    object: &XMIIdReference<EnumWeakCMOF>,
) -> Result<EnumWeakCMOF, anyhow::Error> {
    let object_class = object.get_object()?;
    Ok(object_class)
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
