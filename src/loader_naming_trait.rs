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
#![doc = include_str!("../doc/loader_cmof_structure.md")]

// Package section
use crate::cmof_loader::*;

// Dependencies section

/// Trait for parse naming method to deser_btreemap_using_name_as_key (in loader_deserialise_helper)
pub trait AsNameField {
    /// Parse naming method to deser_btreemap_using_name_as_key (in loader_deserialise_helper)
    fn get_name_field(&self) -> String;
}

impl AsNameField for CMOFPackage {
    fn get_name_field(&self) -> String {
        self.name.clone()
    }
}

impl AsNameField for EnumOwnedMember {
    fn get_name_field(&self) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.xmi_id.label(),
            EnumOwnedMember::Class(content) => content.xmi_id.label(),
            EnumOwnedMember::DataType(content) => content.xmi_id.label(),
            EnumOwnedMember::Enumeration(content) => content.xmi_id.label(),
            EnumOwnedMember::PrimitiveType(content) => content.xmi_id.label(),
        }
    }
}

impl AsNameField for EnumPackageImport {
    fn get_name_field(&self) -> String {
        match self {
            EnumPackageImport::PackageImport(content) => content.xmi_id.label(),
        }
    }
}

impl AsNameField for EnumOwnedEnd {
    fn get_name_field(&self) -> String {
        match self {
            EnumOwnedEnd::Property(content) => content.xmi_id.label(),
        }
    }
}

impl AsNameField for EnumOwnedLiteral {
    fn get_name_field(&self) -> String {
        match self {
            EnumOwnedLiteral::EnumerationLiteral(content) => content.xmi_id.label(),
        }
    }
}

impl AsNameField for EnumOwnedAttribute {
    fn get_name_field(&self) -> String {
        match self {
            EnumOwnedAttribute::Property(content) => content.xmi_id.label(),
        }
    }
}

impl AsNameField for EnumOwnedRule {
    fn get_name_field(&self) -> String {
        match self {
            EnumOwnedRule::Constraint(content) => content.xmi_id.label(),
        }
    }
}
