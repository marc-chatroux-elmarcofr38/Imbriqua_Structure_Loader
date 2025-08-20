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

#[derive(Clone, PartialEq, Debug)]
/// State on package to load
pub enum LoadingState {
    /// No Element (name reserved)
    Empty,
    /// With Element (imported)
    Loaded,
    /// Element used (converted)
    Finished,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, PartialEq, Debug)]
/// Representation of a package
pub struct LoadingPackage {
    /// Source file of the package
    filename: String,
    /// Source id of the package
    id: String,
    /// Json
    cmof_object: Option<Rc<CMOFPackage>>,
    /// State of the package
    state: LoadingState,
    /// Sorted oned_member
    sorted_owned_member: BTreeMap<String, EnumOwnedMember>,
}

impl LoadingPackage {
    /// Instanciate a Loading package
    pub fn new(filename: String, id: String) -> Self {
        LoadingPackage {
            filename,
            id,
            cmof_object: None,
            state: LoadingState::Empty,
            sorted_owned_member: BTreeMap::new(),
        }
    }

    /// State of package
    pub fn get_state(&self) -> &LoadingState {
        &self.state
    }

    /// Label used for identification
    pub fn get_label(&self) -> String {
        let mut label = String::from(&self.filename);
        label.push('#');
        label.push_str(&self.id);
        label
    }

    /// Provide 'object' access control
    pub fn get_json(&self) -> &CMOFPackage {
        if self.state == LoadingState::Loaded {
            if self.cmof_object.is_none() {
                panic!(
                    "Request \"get_json\" on empty json ({:?} status)",
                    self.state
                );
            } else {
                self.cmof_object.as_ref().unwrap()
            }
        } else {
            panic!("Request \"get_json\" on {:?} status", self.state);
        }
    }

    /// Save Element and change state
    pub fn make_loaded(&mut self, cmof: CMOFPackage) {
        self.cmof_object = Some(Rc::new(cmof));
        self.state = LoadingState::Loaded;
    }

    /// Delete Element and change state
    pub fn make_finished(&mut self) -> Result<(), anyhow::Error> {
        self.cmof_object = None;
        self.state = LoadingState::Finished;
        Ok(())
    }
}

impl SetCMOFTools for LoadingPackage {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        if self.cmof_object.is_some() {
            let r = self.cmof_object.as_mut().unwrap();
            let m = Rc::get_mut(r).unwrap();
            m.collect_object(dict_setting, dict_object)?;
            dict_object.insert(m.get_xmi_label()?, EnumCMOF::CMOFPackage(r.clone()));
        } else {
            return Err(anyhow::format_err!("Loading Package without cmof_package"));
        }
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        if self.cmof_object.is_some() {
            let r = self.cmof_object.as_ref().unwrap();
            r.make_post_deserialize(dict_object)?;
        } else {
            return Err(anyhow::format_err!("Loading Package without cmof_package"));
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
