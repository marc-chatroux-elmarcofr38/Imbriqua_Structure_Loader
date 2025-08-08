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

#[derive(Clone, Debug)]
/// List on values necessery for loading but requiring full read of input file for evaluate
pub struct LoadingPreCalculation {
    /// For each owned_member (as model_name format), all name of this package and itself
    /// EX :
    /// "Integer": Named {
    ///     package_name: "dc",
    ///     technical_name: "DC.cmof#Integer",
    ///     table_name: "dc_integer",
    ///     model_name: "Integer",
    ///     full_name: "dc_primitive_integer",
    /// },
    /// "ACorrelationKeyRefCorrelationSubscription": Named {
    ///     package_name: "bpmn_20",
    ///     technical_name: "BPMN20.cmof#A_correlationKeyRef_correlationSubscription",
    ///     table_name: "bpmn_20_a_correlation_key_ref_correlation_subscription",
    ///     model_name: "ACorrelationKeyRefCorrelationSubscription",
    ///     full_name: "bpmn_20_association_a_correlation_key_ref_correlation_subscription",
    /// },
    pub owned_member_type_list: BTreeMap<String, Named>,
    /// For each CMOFassociation (as model_name format), the linked AssociationRelation object
    /// EX :
    /// "A_inputDataRef_inputOutputBinding": AssociationRelation {
    ///     relation_1: ElementRelation {
    ///         element_type: "InputOutputBinding",
    ///         lower: 0,
    ///         upper: Infinity,
    ///     },
    ///     relation_2: ElementRelation {
    ///         element_type: "InputSet",
    ///         lower: 1,
    ///         upper: Finite(
    ///             1,
    ///         ),
    ///     },
    ///     ponteration_type: OneToMany,
    ///     is_self_referencing: false,
    /// },
    // pub association_relation: BTreeMap<String, AssociationRelation>,
    /// For each CMOFClass (as model_name format), all associed CMOFAssociation with rank (provided by association_relation)
    /// EX :
    /// "CorrelationProperty": [
    ///     (
    ///         "A_correlationPropertyRetrievalExpression_correlationproperty",
    ///         IsSecond,
    ///     ),
    ///     (
    ///         "A_type_correlationProperty",
    ///         IsOne,
    ///     ),
    ///     (
    ///         "A_correlationPropertyRef_correlationPropertyBinding",
    ///         IsSecond,
    ///     ),
    ///     (
    ///         "A_correlationPropertyRef_correlationKey",
    ///         IsSecond,
    ///     ),
    /// ],
    // pub association_relation_by_class: BTreeMap<String, Vec<(String, RankRelation)>>,
    /// For each CMOFClass (as model_name format), all CMOFClass (as model_name format) who use it as "Super"
    pub reverse_super_link: BTreeMap<String, Vec<Rc<CMOFClass>>>,
}
impl LoadingPreCalculation {
    /// Create new instance
    pub fn new() -> Self {
        LoadingPreCalculation {
            owned_member_type_list: BTreeMap::new(),
            // association_relation: BTreeMap::new(),
            // association_relation_by_class: BTreeMap::new(),
            reverse_super_link: BTreeMap::new(),
        }
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
