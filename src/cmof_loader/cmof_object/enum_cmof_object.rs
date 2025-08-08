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

use anyhow::Ok;

// Package section
use crate::cmof_loader::*;

// Dependencies section

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
/// All object with xmi_id in CMOF
pub enum EnumCMOF {
    /// CMOFAssociation
    CMOFAssociation(Rc<CMOFAssociation>),
    /// CMOFClass
    CMOFClass(Rc<CMOFClass>),
    /// CMOFConstraint
    CMOFConstraint(Rc<CMOFConstraint>),
    /// CMOFDataType
    CMOFDataType(Rc<CMOFDataType>),
    /// CMOFEnumeration
    CMOFEnumeration(Rc<CMOFEnumeration>),
    /// CMOFEnumerationLiteral
    CMOFEnumerationLiteral(Rc<CMOFEnumerationLiteral>),
    /// CMOFOpaqueExpression
    CMOFOpaqueExpression(Rc<CMOFOpaqueExpression>),
    /// CMOFPackage
    CMOFPackage(Rc<CMOFPackage>),
    /// CMOFPackageImport
    CMOFPackageImport(Rc<CMOFPackageImport>),
    /// CMOFPrimitiveType
    CMOFPrimitiveType(Rc<CMOFPrimitiveType>),
    /// CMOFProperty
    CMOFProperty(Rc<CMOFProperty>),
    /// CMOFTag
    CMOFTag(Rc<CMOFTag>),
}

impl EnumCMOF {
    /// Get "label" of the xmi_id
    pub fn label(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumCMOF::CMOFAssociation(c) => c.xmi_id.label(),
            EnumCMOF::CMOFClass(c) => c.xmi_id.label(),
            EnumCMOF::CMOFConstraint(c) => c.xmi_id.label(),
            EnumCMOF::CMOFDataType(c) => c.xmi_id.label(),
            EnumCMOF::CMOFEnumeration(c) => c.xmi_id.label(),
            EnumCMOF::CMOFEnumerationLiteral(c) => c.xmi_id.label(),
            EnumCMOF::CMOFOpaqueExpression(c) => c.xmi_id.label(),
            EnumCMOF::CMOFPackage(c) => c.xmi_id.label(),
            EnumCMOF::CMOFPackageImport(c) => c.xmi_id.label(),
            EnumCMOF::CMOFPrimitiveType(c) => c.xmi_id.label(),
            EnumCMOF::CMOFProperty(c) => c.xmi_id.label(),
            EnumCMOF::CMOFTag(c) => c.xmi_id.label(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
/// All object with xmi_id in CMOF
pub enum EnumWeakCMOF {
    /// CMOFAssociation
    CMOFAssociation(Weak<CMOFAssociation>),
    /// CMOFClass
    CMOFClass(Weak<CMOFClass>),
    /// CMOFConstraint
    CMOFConstraint(Weak<CMOFConstraint>),
    /// CMOFDataType
    CMOFDataType(Weak<CMOFDataType>),
    /// CMOFEnumeration
    CMOFEnumeration(Weak<CMOFEnumeration>),
    /// CMOFEnumerationLiteral
    CMOFEnumerationLiteral(Weak<CMOFEnumerationLiteral>),
    /// CMOFOpaqueExpression
    CMOFOpaqueExpression(Weak<CMOFOpaqueExpression>),
    /// CMOFPackage
    CMOFPackage(Weak<CMOFPackage>),
    /// CMOFPackageImport
    CMOFPackageImport(Weak<CMOFPackageImport>),
    /// CMOFPrimitiveType
    CMOFPrimitiveType(Weak<CMOFPrimitiveType>),
    /// CMOFProperty
    CMOFProperty(Weak<CMOFProperty>),
    /// CMOFTag
    CMOFTag(Weak<CMOFTag>),
}

impl EnumWeakCMOF {
    pub fn from(object: &EnumCMOF) -> Self {
        match object {
            EnumCMOF::CMOFAssociation(c) => EnumWeakCMOF::CMOFAssociation(Rc::downgrade(&c)),
            EnumCMOF::CMOFClass(c) => EnumWeakCMOF::CMOFClass(Rc::downgrade(&c)),
            EnumCMOF::CMOFConstraint(c) => EnumWeakCMOF::CMOFConstraint(Rc::downgrade(&c)),
            EnumCMOF::CMOFDataType(c) => EnumWeakCMOF::CMOFDataType(Rc::downgrade(&c)),
            EnumCMOF::CMOFEnumeration(c) => EnumWeakCMOF::CMOFEnumeration(Rc::downgrade(&c)),
            EnumCMOF::CMOFEnumerationLiteral(c) => {
                EnumWeakCMOF::CMOFEnumerationLiteral(Rc::downgrade(&c))
            }
            EnumCMOF::CMOFOpaqueExpression(c) => {
                EnumWeakCMOF::CMOFOpaqueExpression(Rc::downgrade(&c))
            }
            EnumCMOF::CMOFPackage(c) => EnumWeakCMOF::CMOFPackage(Rc::downgrade(&c)),
            EnumCMOF::CMOFPackageImport(c) => EnumWeakCMOF::CMOFPackageImport(Rc::downgrade(&c)),
            EnumCMOF::CMOFPrimitiveType(c) => EnumWeakCMOF::CMOFPrimitiveType(Rc::downgrade(&c)),
            EnumCMOF::CMOFProperty(c) => EnumWeakCMOF::CMOFProperty(Rc::downgrade(&c)),
            EnumCMOF::CMOFTag(c) => EnumWeakCMOF::CMOFTag(Rc::downgrade(&c)),
        }
    }

    pub fn upgrade(&self) -> Result<EnumCMOF, anyhow::Error> {
        match self {
            EnumWeakCMOF::CMOFAssociation(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFAssociation(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            EnumWeakCMOF::CMOFClass(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFClass(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            EnumWeakCMOF::CMOFConstraint(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFConstraint(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            EnumWeakCMOF::CMOFDataType(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFDataType(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            EnumWeakCMOF::CMOFEnumeration(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFEnumeration(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            EnumWeakCMOF::CMOFEnumerationLiteral(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFEnumerationLiteral(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            EnumWeakCMOF::CMOFOpaqueExpression(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFOpaqueExpression(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            EnumWeakCMOF::CMOFPackage(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFPackage(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            EnumWeakCMOF::CMOFPackageImport(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFPackageImport(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            EnumWeakCMOF::CMOFPrimitiveType(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFPrimitiveType(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            EnumWeakCMOF::CMOFProperty(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFProperty(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
            EnumWeakCMOF::CMOFTag(c) => {
                let r = c.upgrade();
                match r {
                    Some(content) => Ok(EnumCMOF::CMOFTag(content)),
                    None => Err(anyhow::format_err!("Upgrade result 'None' : {:?}", self)),
                }
            }
        }
    }
}

impl XMIIdReference<EnumWeakCMOF> {
    /// Push content to RefCell in XMIIDReerence
    pub fn set_xmi_id_object(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Criteria
        if self.get_object().is_ok() {
            panic!("'{:#?}' is already loaded", self)
        };

        // Catch
        let k = self.label()?;
        let r = dict_object.get(&k);
        if r.is_none() {
            return Err(anyhow::format_err!(
                "Matching error in post_deserialize : \"{}\" not find in dict_object",
                k
            ));
        } else {
            let v = r.unwrap();
            match v {
                EnumCMOF::CMOFAssociation(c) => {
                    self.set_object(EnumWeakCMOF::CMOFAssociation(Rc::downgrade(c)));
                }
                EnumCMOF::CMOFClass(c) => {
                    self.set_object(EnumWeakCMOF::CMOFClass(Rc::downgrade(c)));
                }
                EnumCMOF::CMOFConstraint(c) => {
                    self.set_object(EnumWeakCMOF::CMOFConstraint(Rc::downgrade(c)));
                }
                EnumCMOF::CMOFDataType(c) => {
                    self.set_object(EnumWeakCMOF::CMOFDataType(Rc::downgrade(c)));
                }
                EnumCMOF::CMOFEnumeration(c) => {
                    self.set_object(EnumWeakCMOF::CMOFEnumeration(Rc::downgrade(c)));
                }
                EnumCMOF::CMOFEnumerationLiteral(c) => {
                    self.set_object(EnumWeakCMOF::CMOFEnumerationLiteral(Rc::downgrade(c)));
                }
                EnumCMOF::CMOFOpaqueExpression(c) => {
                    self.set_object(EnumWeakCMOF::CMOFOpaqueExpression(Rc::downgrade(c)));
                }
                EnumCMOF::CMOFPackage(c) => {
                    self.set_object(EnumWeakCMOF::CMOFPackage(Rc::downgrade(c)));
                }
                EnumCMOF::CMOFPackageImport(c) => {
                    self.set_object(EnumWeakCMOF::CMOFPackageImport(Rc::downgrade(c)));
                }
                EnumCMOF::CMOFPrimitiveType(c) => {
                    self.set_object(EnumWeakCMOF::CMOFPrimitiveType(Rc::downgrade(c)));
                }
                EnumCMOF::CMOFProperty(c) => {
                    self.set_object(EnumWeakCMOF::CMOFProperty(Rc::downgrade(c)));
                }
                EnumCMOF::CMOFTag(c) => {
                    self.set_object(EnumWeakCMOF::CMOFTag(Rc::downgrade(c)));
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

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedAttribute Tag
pub enum EnumOwnedAttribute {
    /// OwnedAttribute with cmof:Property type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Property")]
    Property(Rc<CMOFProperty>),
}

// impl SetCMOFTools for EnumOwnedAttribute {
//     fn collect_object(
//         &mut self,
//         dict_setting: &mut BTreeMap<String, String>,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumOwnedAttribute::Property(c) => {
//                 let m = Rc::get_mut(c).unwrap();
//                 m.collect_object(dict_setting, dict_object)?;
//                 dict_object.insert(c.get_xmi_id_field()?, EnumCMOF::CMOFProperty(c.clone()));
//             }
//         }
//         Ok(())
//     }

//     fn make_post_deserialize(
//         &self,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumOwnedAttribute::Property(c) => catch_and_make_post_deserialize(c, dict_object),
//         }
//     }
// }

impl GetXMIId for EnumOwnedAttribute {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumOwnedAttribute::Property(c) => c.get_xmi_id_field(),
        }
    }
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumOwnedAttribute::Property(c) => c.get_xmi_id_object(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedEnd Tag
pub enum EnumOwnedEnd {
    /// OwnedEnd with cmof:Property type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Property")]
    Property(Rc<CMOFProperty>),
}

// impl SetCMOFTools for EnumOwnedEnd {
//     fn collect_object(
//         &mut self,
//         dict_setting: &mut BTreeMap<String, String>,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumOwnedEnd::Property(c) => {
//                 let m = Rc::get_mut(c).unwrap();
//                 m.collect_object(dict_setting, dict_object)?;
//                 dict_object.insert(c.get_xmi_id_field()?, EnumCMOF::CMOFProperty(c.clone()));
//             }
//         }
//         Ok(())
//     }

//     fn make_post_deserialize(
//         &self,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumOwnedEnd::Property(c) => catch_and_make_post_deserialize(c, dict_object),
//         }
//     }
// }

impl GetXMIId for EnumOwnedEnd {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumOwnedEnd::Property(c) => c.get_xmi_id_field(),
        }
    }
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumOwnedEnd::Property(c) => c.get_xmi_id_object(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedLiteral Tag
pub enum EnumOwnedLiteral {
    /// OwnedLiteral with cmof:EnumerationLiteral type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:EnumerationLiteral")]
    EnumerationLiteral(Rc<CMOFEnumerationLiteral>),
}

// impl SetCMOFTools for EnumOwnedLiteral {
//     fn collect_object(
//         &mut self,
//         dict_setting: &mut BTreeMap<String, String>,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumOwnedLiteral::EnumerationLiteral(c) => {
//                 catch_and_collect_object(c, dict_setting, dict_object)
//             }
//         }
//     }

//     fn make_post_deserialize(
//         &self,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumOwnedLiteral::EnumerationLiteral(c) => {
//                 catch_and_make_post_deserialize(c, dict_object)
//             }
//         }
//     }
// }

impl GetXMIId for EnumOwnedLiteral {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumOwnedLiteral::EnumerationLiteral(c) => c.get_xmi_id_field(),
        }
    }
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumOwnedLiteral::EnumerationLiteral(c) => c.get_xmi_id_object(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedMember Tag
pub enum EnumOwnedMember {
    /// OwnedMember with cmof:Association type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Association")]
    Association(Rc<CMOFAssociation>),
    /// OwnedMember with cmof:Class type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Class")]
    Class(Rc<CMOFClass>),
    /// OwnedMember with cmof:DataType type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:DataType")]
    DataType(Rc<CMOFDataType>),
    /// OwnedMember with cmof:Enumeration type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Enumeration")]
    Enumeration(Rc<CMOFEnumeration>),
    /// OwnedMember with cmof:PrimitiveType type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:PrimitiveType")]
    PrimitiveType(Rc<CMOFPrimitiveType>),
}

// impl SetCMOFTools for EnumOwnedMember {
//     fn collect_object(
//         &mut self,
//         dict_setting: &mut BTreeMap<String, String>,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumOwnedMember::Association(c) => {
//                 catch_and_collect_object(c, dict_setting, dict_object)
//             }
//             EnumOwnedMember::Class(c) => catch_and_collect_object(c, dict_setting, dict_object),
//             EnumOwnedMember::DataType(c) => catch_and_collect_object(c, dict_setting, dict_object),
//             EnumOwnedMember::Enumeration(c) => {
//                 catch_and_collect_object(c, dict_setting, dict_object)
//             }
//             EnumOwnedMember::PrimitiveType(c) => {
//                 catch_and_collect_object(c, dict_setting, dict_object)
//             }
//         }
//     }

//     fn make_post_deserialize(
//         &self,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumOwnedMember::Association(c) => catch_and_make_post_deserialize(c, dict_object),
//             EnumOwnedMember::Class(c) => catch_and_make_post_deserialize(c, dict_object),
//             EnumOwnedMember::DataType(c) => catch_and_make_post_deserialize(c, dict_object),
//             EnumOwnedMember::Enumeration(c) => catch_and_make_post_deserialize(c, dict_object),
//             EnumOwnedMember::PrimitiveType(c) => catch_and_make_post_deserialize(c, dict_object),
//         }
//     }
// }

impl GetXMIId for EnumOwnedMember {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumOwnedMember::Association(c) => c.get_xmi_id_field(),
            EnumOwnedMember::Class(c) => c.get_xmi_id_field(),
            EnumOwnedMember::DataType(c) => c.get_xmi_id_field(),
            EnumOwnedMember::Enumeration(c) => c.get_xmi_id_field(),
            EnumOwnedMember::PrimitiveType(c) => c.get_xmi_id_field(),
        }
    }
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumOwnedMember::Association(c) => c.get_xmi_id_object(),
            EnumOwnedMember::Class(c) => c.get_xmi_id_object(),
            EnumOwnedMember::DataType(c) => c.get_xmi_id_object(),
            EnumOwnedMember::Enumeration(c) => c.get_xmi_id_object(),
            EnumOwnedMember::PrimitiveType(c) => c.get_xmi_id_object(),
        }
    }
}

impl EnumOwnedMember {
    pub fn get_technical_name(&self) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.technical_name.clone(),
            EnumOwnedMember::Class(content) => content.technical_name.clone(),
            EnumOwnedMember::DataType(content) => content.technical_name.clone(),
            EnumOwnedMember::Enumeration(content) => content.technical_name.clone(),
            EnumOwnedMember::PrimitiveType(content) => content.technical_name.clone(),
        }
    }
    pub fn get_table_name(&self) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.table_name.clone(),
            EnumOwnedMember::Class(content) => content.table_name.clone(),
            EnumOwnedMember::DataType(content) => content.table_name.clone(),
            EnumOwnedMember::Enumeration(content) => content.table_name.clone(),
            EnumOwnedMember::PrimitiveType(content) => content.table_name.clone(),
        }
    }
    pub fn get_model_name(&self) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.model_name.clone(),
            EnumOwnedMember::Class(content) => content.model_name.clone(),
            EnumOwnedMember::DataType(content) => content.model_name.clone(),
            EnumOwnedMember::Enumeration(content) => content.model_name.clone(),
            EnumOwnedMember::PrimitiveType(content) => content.model_name.clone(),
        }
    }
    pub fn get_full_name(&self) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.full_name.clone(),
            EnumOwnedMember::Class(content) => content.full_name.clone(),
            EnumOwnedMember::DataType(content) => content.full_name.clone(),
            EnumOwnedMember::Enumeration(content) => content.full_name.clone(),
            EnumOwnedMember::PrimitiveType(content) => content.full_name.clone(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedRule Tag
pub enum EnumOwnedRule {
    /// OwnedRule with cmof:Constraint type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Constraint")]
    Constraint(Rc<CMOFConstraint>),
}

// impl SetCMOFTools for EnumOwnedRule {
//     fn collect_object(
//         &mut self,
//         dict_setting: &mut BTreeMap<String, String>,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumOwnedRule::Constraint(c) => {
//                 let m = Rc::get_mut(c).unwrap();
//                 m.collect_object(dict_setting, dict_object)?;
//                 dict_object.insert(c.get_xmi_id_field()?, EnumCMOF::CMOFConstraint(c.clone()));
//             }
//         }
//         Ok(())
//     }

//     fn make_post_deserialize(
//         &self,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumOwnedRule::Constraint(c) => catch_and_make_post_deserialize(c, dict_object),
//         }
//     }
// }

impl GetXMIId for EnumOwnedRule {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumOwnedRule::Constraint(c) => c.get_xmi_id_field(),
        }
    }
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumOwnedRule::Constraint(c) => c.get_xmi_id_object(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing PackageImport Tag
pub enum EnumPackageImport {
    /// PackageImport Tag with cmof:PackageImport type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:PackageImport")]
    PackageImport(Rc<CMOFPackageImport>),
}

// impl SetCMOFTools for EnumPackageImport {
//     fn collect_object(
//         &mut self,
//         dict_setting: &mut BTreeMap<String, String>,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumPackageImport::PackageImport(c) => {
//                 let m = Rc::get_mut(c).unwrap();
//                 m.collect_object(dict_setting, dict_object)?;
//                 dict_object.insert(
//                     c.get_xmi_id_field()?,
//                     EnumCMOF::CMOFPackageImport(c.clone()),
//                 );
//             }
//         }
//         Ok(())
//     }

//     fn make_post_deserialize(
//         &self,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumPackageImport::PackageImport(c) => catch_and_make_post_deserialize(c, dict_object),
//         }
//     }
// }

impl GetXMIId for EnumPackageImport {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumPackageImport::PackageImport(c) => c.get_xmi_id_field(),
        }
    }
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumPackageImport::PackageImport(c) => c.get_xmi_id_object(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Specification Tag
pub enum EnumSpecification {
    /// Specification Tag with cmof:OpaqueExpression type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:OpaqueExpression")]
    OpaqueExpression(Rc<CMOFOpaqueExpression>),
}

// impl SetCMOFTools for EnumSpecification {
//     fn collect_object(
//         &mut self,
//         dict_setting: &mut BTreeMap<String, String>,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumSpecification::OpaqueExpression(c) => {
//                 let m = Rc::get_mut(c).unwrap();
//                 m.collect_object(dict_setting, dict_object)?;
//                 dict_object.insert(
//                     c.get_xmi_id_field()?,
//                     EnumCMOF::CMOFOpaqueExpression(c.clone()),
//                 );
//             }
//         }
//         Ok(())
//     }

//     fn make_post_deserialize(
//         &self,
//         dict_object: &mut BTreeMap<String, EnumCMOF>,
//     ) -> Result<(), anyhow::Error> {
//         match self {
//             EnumSpecification::OpaqueExpression(c) => {
//                 catch_and_make_post_deserialize(c, dict_object)
//             }
//         }
//     }
// }

impl GetXMIId for EnumSpecification {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumSpecification::OpaqueExpression(c) => c.get_xmi_id_field(),
        }
    }
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumSpecification::OpaqueExpression(c) => c.get_xmi_id_object(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

fn catch_and_collect_object<T: SetCMOFTools>(
    object: &mut Rc<T>,
    dict_setting: &mut BTreeMap<String, String>,
    dict_object: &mut BTreeMap<String, EnumCMOF>,
) -> Result<(), anyhow::Error> {
    let m = Rc::get_mut(object);
    if m.is_some() {
        m.unwrap().collect_object(dict_setting, dict_object)?;
        Ok(())
    } else {
        Err(anyhow::format_err!(
            "Error getting Mut of a Rc (catch_and_collect_object)"
        ))
    }
}

// ####################################################################################################
//
// ####################################################################################################

fn catch_and_make_post_deserialize<T: SetCMOFTools + GetXMIId>(
    object: &Rc<T>,
    dict_object: &mut BTreeMap<String, EnumCMOF>,
) -> Result<(), anyhow::Error> {
    object.make_post_deserialize(dict_object)?;
    Ok(())
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
