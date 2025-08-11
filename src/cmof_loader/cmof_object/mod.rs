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
#![doc = include_str!("mod.md")]

// Mod section
mod cmof_association;
mod cmof_class;
mod cmof_constraint;
mod cmof_datatype;
mod cmof_enumeration;
mod cmof_enumeration_literal;
mod cmof_opaque_expression;
mod cmof_package;
mod cmof_package_import;
mod cmof_primitive_type;
mod cmof_property;
mod cmof_tag;
mod enum_cmof_object;
mod enum_href_object;
mod file_cmof;
mod set_xmi_id_object;
mod uml_visibility_kind;
pub use cmof_association::*;
pub use cmof_class::*;
pub use cmof_constraint::*;
pub use cmof_datatype::*;
pub use cmof_enumeration::*;
pub use cmof_enumeration_literal::*;
pub use cmof_opaque_expression::*;
pub use cmof_package::*;
pub use cmof_package_import::*;
pub use cmof_primitive_type::*;
pub use cmof_property::*;
pub use cmof_tag::*;
pub use enum_cmof_object::*;
pub use enum_href_object::*;
pub use file_cmof::*;
pub use set_xmi_id_object::*;
pub use uml_visibility_kind::*;
