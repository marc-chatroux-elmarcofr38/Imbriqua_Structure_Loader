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

// ####################################################################################################
//
// ####################################################################################################

/// Trait for complete an object containing XMI Id Reference in HRef***** structure
/// Input :
///     - BTreeMap of all potential object content
pub trait SetXMIIdObject {
    /// Function for complete an object containing XMI Id Reference in HRef***** structure
    /// Input :
    ///     - BTreeMap of all potential object content
    fn set_xmi_id_object(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error>;
}

// ####################################################################################################
//
// ####################################################################################################

impl SetXMIIdObject for HRefRedefinedProperty {
    fn set_xmi_id_object(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Criteria
        if self.href.get_object().is_ok() {
            panic!("'{:#?}' is already loaded", &self.href)
        };

        // Catch
        let k = self.href.label()?;
        let r = dict_object.get(&k);
        if r.is_none() {
            return Err(anyhow::format_err!(
                "Matching error in post_deserialize : \"{}\" not find in dict_object",
                k
            ));
        } else {
            let v = r.unwrap();
            match v {
                EnumCMOF::CMOFProperty(c) => {
                    self.href.set_object(Rc::downgrade(c));
                }
                _ => {
                    return Err(anyhow::format_err!(
                        "Unexpected type for '{}' (require CMOFProperty reference only, HRefRedefinedProperty)",
                        k
                    ));
                }
            }
        }
        // Return
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetXMIIdObject for HRefSubsettedProperty {
    fn set_xmi_id_object(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Criteria
        if self.href.get_object().is_ok() {
            panic!("'{:#?}' is already loaded", &self.href)
        };

        // Catch
        let k = self.href.label()?;
        let r = dict_object.get(&k);
        if r.is_none() {
            return Err(anyhow::format_err!(
                "Matching error in post_deserialize : \"{}\" not find in dict_object",
                k
            ));
        } else {
            let v = r.unwrap();
            match v {
                EnumCMOF::CMOFProperty(c) => {
                    self.href.set_object(Rc::downgrade(c));
                }
                _ => {
                    return Err(anyhow::format_err!(
                        "Unexpected type for '{}' (require CMOFProperty reference only, HRefSubsettedProperty)",
                        k
                    ));
                }
            }
        }
        // Return
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetXMIIdObject for HRefSuperClass {
    fn set_xmi_id_object(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Criteria
        if self.href.get_object().is_ok() {
            panic!("'{:#?}' is already loaded", &self.href)
        };

        // Catch
        let k = self.href.label()?;
        let r = dict_object.get(&k);
        if r.is_none() {
            return Err(anyhow::format_err!(
                "Matching error in post_deserialize : \"{}\" not find in dict_object",
                k
            ));
        } else {
            let v = r.unwrap();
            match v {
                EnumCMOF::CMOFClass(c) => {
                    self.href.set_object(Rc::downgrade(c));
                }
                _ => {
                    return Err(anyhow::format_err!(
                        "Unexpected type for '{}' (require CMOFPackageImport reference only, HRefSuperClass)",
                        k
                    ));
                }
            }
        }
        // Return
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetXMIIdObject for HRefImportedPackage {
    fn set_xmi_id_object(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Criteria
        if self.href.get_object().is_ok() {
            panic!("'{:#?}' is already loaded", &self.href)
        };

        // Catch
        let k = self.href.label()?;
        let r = dict_object.get(&k);
        if r.is_none() {
            return Err(anyhow::format_err!(
                "Matching error in post_deserialize : \"{}\" not find in dict_object",
                k
            ));
        } else {
            let v = r.unwrap();
            match v {
                EnumCMOF::CMOFPackage(c) => {
                    self.href.set_object(Rc::downgrade(c));
                }
                _ => {
                    return Err(anyhow::format_err!(
                        "Unexpected type for '{}' (require CMOFPackage reference only, HRefImportedPackage)",
                        k
                    ));
                }
            }
        }
        // Return
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetXMIIdObject for HRefClass {
    fn set_xmi_id_object(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Criteria
        if self.href.get_object().is_ok() {
            panic!("'{:#?}' is already loaded", &self.href)
        };

        // Catch
        let k = self.href.label()?;
        let r = dict_object.get(&k);
        if r.is_none() {
            return Err(anyhow::format_err!(
                "Matching error in post_deserialize : \"{}\" not find in dict_object",
                k
            ));
        } else {
            let v = r.unwrap();
            match v {
                EnumCMOF::CMOFClass(c) => {
                    self.href.set_object(Rc::downgrade(c));
                }
                _ => {
                    return Err(anyhow::format_err!(
                        "Unexpected type for '{}' (require CMOFClass reference only, HRefClass)",
                        k
                    ));
                }
            }
        }
        // Return
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetXMIIdObject for HRefPrimitiveType {
    fn set_xmi_id_object(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Criteria
        if self.href.get_object().is_ok() {
            panic!("'{:#?}' is already loaded", &self.href)
        };

        // Catch
        let k = self.href.label()?;
        let r = dict_object.get(&k);
        if r.is_none() {
            return Err(anyhow::format_err!(
                "Matching error in post_deserialize : \"{}\" not find in dict_object",
                k
            ));
        } else {
            let v = r.unwrap();
            match v {
                EnumCMOF::CMOFPrimitiveType(c) => {
                    self.href.set_object(Rc::downgrade(c));
                }
                _ => {
                    return Err(anyhow::format_err!("Unexpected type for '{}' (require CMOFPrimitiveType reference only, HRefPrimitiveType)", k));
                }
            }
        }
        // Return
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetXMIIdObject for HRefDataType {
    fn set_xmi_id_object(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Criteria
        if self.href.get_object().is_ok() {
            panic!("'{:#?}' is already loaded", &self.href)
        };

        // Catch
        let k = self.href.label()?;
        let r = dict_object.get(&k);
        if r.is_none() {
            return Err(anyhow::format_err!(
                "Matching error in post_deserialize : \"{}\" not find in dict_object",
                k
            ));
        } else {
            let v = r.unwrap();
            match v {
                EnumCMOF::CMOFDataType(c) => {
                    self.href.set_object(Rc::downgrade(c));
                }
                _ => {
                    return Err(anyhow::format_err!("Unexpected type for '{}' (require CMOFDataType reference only, HRefDataType)", k));
                }
            }
        }
        // Return
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
