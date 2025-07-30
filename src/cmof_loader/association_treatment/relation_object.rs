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
/// Pre Calculation struct helping loading CMOFAssociation
pub struct AssociationRelation {
    /// First relation
    pub relation_1: Rc<CMOFProperty>,
    /// Second relation
    pub relation_2: Rc<CMOFProperty>,
    /// Ponderation of the relation
    pub ponteration_type: RelationPonderationType,
    /// if is itself reference
    pub is_self_referencing: bool,
}

// ####################################################################################################
//
// ####################################################################################################

impl AssociationRelation {
    pub fn reverse(&self) -> Self {
        AssociationRelation {
            relation_1: self.relation_2.clone(),
            relation_2: self.relation_1.clone(),
            ponteration_type: self.ponteration_type.clone(),
            is_self_referencing: self.is_self_referencing,
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, PartialEq, Debug)]
/// Help for AssociationRelation
pub enum RelationPonderationType {
    /// one relation_1 need one relation_2
    OneToOne,
    /// many relation_1 need one relation_2
    OneToMany,
    /// many relation_1 need many relation_2
    ManyToMany,
}

#[derive(Clone, PartialEq, Debug)]
/// Help for AssociationRelation pivot
pub enum RelationSource {
    /// If is fromm CMOFAssociation
    FromAssociation,
    /// If is from CMOFClass
    FromClass,
}

#[derive(Clone, PartialEq, Debug)]
/// Help for AssociationRelation pivot
pub enum RankRelation {
    /// Is relation_1
    IsOne,
    /// Is relation_2
    IsSecond,
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
pub enum Relation {
    // Relation : One 'from' as One 'to'
    OneToOneRelation(OneToOneRelation),
    // Relation : Many 'from' as One 'to'
    OneToManyRelation(OneToManyRelation),
    // Relation : Many 'from' as Many 'to'
    ManToManyRelation(ManToManyRelation),
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
pub struct OneToOneRelation {
    from: Rc<CMOFProperty>,
    to: Rc<CMOFProperty>,
}

impl OneToOneRelation {
    pub fn new(from: Rc<CMOFProperty>, to: Rc<CMOFProperty>) -> Self {
        OneToOneRelation { from: from, to: to }
    }
    pub fn get_from(&self) -> Rc<CMOFProperty> {
        self.from.clone()
    }
    pub fn get_to(&self) -> Rc<CMOFProperty> {
        self.to.clone()
    }
    pub fn get_from_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        self.from.parent.get_object_as_class()
    }
    pub fn get_to_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        self.to.parent.get_object_as_class()
    }
    pub fn is_self_referencing(&self) -> Result<bool, anyhow::Error> {
        let from_id = self.from.parent.label()?;
        let to_id = self.to.parent.label()?;
        Ok(from_id == to_id)
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
pub struct OneToManyRelation {
    from: Rc<CMOFProperty>,
    to: Rc<CMOFProperty>,
}

impl OneToManyRelation {
    pub fn new(from: Rc<CMOFProperty>, to: Rc<CMOFProperty>) -> Self {
        OneToManyRelation { from: from, to: to }
    }
    pub fn get_from(&self) -> Rc<CMOFProperty> {
        self.from.clone()
    }
    pub fn get_to(&self) -> Rc<CMOFProperty> {
        self.to.clone()
    }
    pub fn get_from_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        self.from.parent.get_object_as_class()
    }
    pub fn get_to_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        self.to.parent.get_object_as_class()
    }
    pub fn is_self_referencing(&self) -> Result<bool, anyhow::Error> {
        let from_id = self.from.parent.label()?;
        let to_id = self.to.parent.label()?;
        Ok(from_id == to_id)
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
pub struct ManToManyRelation {
    from: Rc<CMOFProperty>,
    to: Rc<CMOFProperty>,
}

impl ManToManyRelation {
    pub fn new(from: Rc<CMOFProperty>, to: Rc<CMOFProperty>) -> Self {
        ManToManyRelation { from: from, to: to }
    }
    pub fn get_from(&self) -> Rc<CMOFProperty> {
        self.from.clone()
    }
    pub fn get_to(&self) -> Rc<CMOFProperty> {
        self.to.clone()
    }
    pub fn get_from_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        self.from.parent.get_object_as_class()
    }
    pub fn get_to_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        self.to.parent.get_object_as_class()
    }
    pub fn is_self_referencing(&self) -> Result<bool, anyhow::Error> {
        let from_id = self.from.parent.label()?;
        let to_id = self.to.parent.label()?;
        Ok(from_id == to_id)
    }
}
