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

/// Tools for Relation use
pub trait RelationTools {
    /// Get 'from' property
    fn get_from(&self) -> Rc<CMOFProperty>;
    /// Get 'to' property
    fn get_to(&self) -> Rc<CMOFProperty>;
    /// Get class associated with 'from' property
    fn get_from_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error>;
    /// Get class associated with 'to' property
    fn get_to_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error>;
    /// Is 'from' and 'to' class are the same
    fn is_self_referencing(&self) -> Result<bool, anyhow::Error>;
    /// Check if characteristic are good
    fn check(&self) -> Result<(), anyhow::Error>;
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
/// Relation representation
pub enum Relation {
    /// Relation : One 'from' as One 'to'
    OneToOneRelation(OneToOneRelation),
    /// Relation : Many 'from' as One 'to'
    OneToManyRelation(OneToManyRelation),
    /// Relation : Many 'from' as Many 'to'
    ManyToManyRelation(ManyToManyRelation),
}

impl RelationTools for Relation {
    fn get_from(&self) -> Rc<CMOFProperty> {
        match self {
            Relation::OneToOneRelation(c) => c.get_from(),
            Relation::OneToManyRelation(c) => c.get_from(),
            Relation::ManyToManyRelation(c) => c.get_from(),
        }
    }
    fn get_to(&self) -> Rc<CMOFProperty> {
        match self {
            Relation::OneToOneRelation(c) => c.get_to(),
            Relation::OneToManyRelation(c) => c.get_to(),
            Relation::ManyToManyRelation(c) => c.get_to(),
        }
    }
    fn get_from_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        match self {
            Relation::OneToOneRelation(c) => c.get_from_class(),
            Relation::OneToManyRelation(c) => c.get_from_class(),
            Relation::ManyToManyRelation(c) => c.get_from_class(),
        }
    }
    fn get_to_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        match self {
            Relation::OneToOneRelation(c) => c.get_to_class(),
            Relation::OneToManyRelation(c) => c.get_to_class(),
            Relation::ManyToManyRelation(c) => c.get_to_class(),
        }
    }
    fn is_self_referencing(&self) -> Result<bool, anyhow::Error> {
        match self {
            Relation::OneToOneRelation(c) => c.is_self_referencing(),
            Relation::OneToManyRelation(c) => c.is_self_referencing(),
            Relation::ManyToManyRelation(c) => c.is_self_referencing(),
        }
    }
    fn check(&self) -> Result<(), anyhow::Error> {
        match self {
            Relation::OneToOneRelation(c) => c.check(),
            Relation::OneToManyRelation(c) => c.check(),
            Relation::ManyToManyRelation(c) => c.check(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
/// Relation : One 'from' as One 'to'
pub struct OneToOneRelation {
    from: Rc<CMOFProperty>,
    to: Rc<CMOFProperty>,
}

impl OneToOneRelation {
    /// New Relation : One 'from' as One 'to'
    pub fn new(from: Rc<CMOFProperty>, to: Rc<CMOFProperty>) -> Result<Self, anyhow::Error> {
        let r = OneToOneRelation { from: from, to: to };
        r.check()?;
        Ok(r)
    }
}

impl RelationTools for OneToOneRelation {
    fn get_from(&self) -> Rc<CMOFProperty> {
        self.from.clone()
    }
    fn get_to(&self) -> Rc<CMOFProperty> {
        self.to.clone()
    }
    fn get_from_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        get_object_as_class(&self.from.parent)
    }
    fn get_to_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        get_object_as_class(&self.to.parent)
    }
    fn is_self_referencing(&self) -> Result<bool, anyhow::Error> {
        let from_id = self.from.parent.label()?;
        let to_id = self.to.parent.label()?;
        Ok(from_id == to_id)
    }
    fn check(&self) -> Result<(), anyhow::Error> {
        // Criteria 1 : 'from' ponderation
        if self.from.upper > infinitable::Finite(1) {
            return Err(anyhow::format_err!(
                "OneToOneRelation Check : Criteria 1 : 'from' ponderation (upper)"
            ));
        }

        // Criteria 2 : 'to' ponderation
        if self.to.upper > infinitable::Finite(1) {
            return Err(anyhow::format_err!(
                "OneToOneRelation Check : Criteria 2 : 'to' ponderation (upper)"
            ));
        }

        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
/// Relation : Many 'from' as One 'to'
pub struct OneToManyRelation {
    from: Rc<CMOFProperty>,
    to: Rc<CMOFProperty>,
}

impl OneToManyRelation {
    /// New Relation : Many 'from' as One 'to'
    pub fn new(from: Rc<CMOFProperty>, to: Rc<CMOFProperty>) -> Result<Self, anyhow::Error> {
        let r = OneToManyRelation { from: from, to: to };
        r.check()?;
        Ok(r)
    }
}

impl RelationTools for OneToManyRelation {
    fn get_from(&self) -> Rc<CMOFProperty> {
        self.from.clone()
    }
    fn get_to(&self) -> Rc<CMOFProperty> {
        self.to.clone()
    }
    fn get_from_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        get_object_as_class(&self.from.parent)
    }
    fn get_to_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        get_object_as_class(&self.to.parent)
    }
    fn is_self_referencing(&self) -> Result<bool, anyhow::Error> {
        let from_id = self.from.parent.label()?;
        let to_id = self.to.parent.label()?;
        Ok(from_id == to_id)
    }
    fn check(&self) -> Result<(), anyhow::Error> {
        // Criteria 1 : 'from' ponderation
        if self.from.upper < infinitable::Finite(2) {
            return Err(anyhow::format_err!(
                "OneToManyRelation Check : Criteria 1 : 'from' ponderation (upper)"
            ));
        }

        // Criteria 2 : 'to' ponderation
        if self.to.upper > infinitable::Finite(1) {
            return Err(anyhow::format_err!(
                "OneToManyRelation Check : Criteria 2 : 'to' ponderation (upper)"
            ));
        }

        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
/// Relation : Many 'from' as Many 'to'
pub struct ManyToManyRelation {
    from: Rc<CMOFProperty>,
    to: Rc<CMOFProperty>,
}

impl ManyToManyRelation {
    /// New Relation : Many 'from' as Many 'to'
    pub fn new(from: Rc<CMOFProperty>, to: Rc<CMOFProperty>) -> Result<Self, anyhow::Error> {
        let r = ManyToManyRelation { from: from, to: to };
        r.check()?;
        Ok(r)
    }
}

impl RelationTools for ManyToManyRelation {
    fn get_from(&self) -> Rc<CMOFProperty> {
        self.from.clone()
    }
    fn get_to(&self) -> Rc<CMOFProperty> {
        self.to.clone()
    }
    fn get_from_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        get_object_as_class(&self.from.parent)
    }
    fn get_to_class(&self) -> Result<Rc<CMOFClass>, anyhow::Error> {
        get_object_as_class(&self.to.parent)
    }
    fn is_self_referencing(&self) -> Result<bool, anyhow::Error> {
        let from_id = self.from.parent.label()?;
        let to_id = self.to.parent.label()?;
        Ok(from_id == to_id)
    }
    fn check(&self) -> Result<(), anyhow::Error> {
        // Criteria 1 : 'from' ponderation
        if self.from.upper < infinitable::Finite(2) {
            return Err(anyhow::format_err!(
                "ManyToManyRelation Check : Criteria 1 : 'from' ponderation (upper)"
            ));
        }

        // Criteria 2 : 'to' ponderation
        if self.to.upper < infinitable::Finite(2) {
            return Err(anyhow::format_err!(
                "ManyToManyRelation Check : Criteria 2 : 'to' ponderation (upper)"
            ));
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
