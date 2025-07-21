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

#[derive(Clone, PartialEq, Debug)]
/// State on package to load
pub enum ClassClassification {
    /// Primal : depend of nothing
    Primal,
    /// Simple : don't need lifetime for utilization, but need reference
    Simple,
    /// Complex : need lifetime for utilization
    Complex,
}
