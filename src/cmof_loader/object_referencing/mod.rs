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
// #![doc = include_str!("../../../doc/loader_deserialise_helper.md")]

// Mod section
mod href_object;
mod xmi_reference;
pub use href_object::*;
pub use xmi_reference::*;

// ####################################################################################################
//
// ####################################################################################################

#[cfg(test)]
mod xmi_reference_test {
    use super::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn local_reference_01_creation() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let _ = XMIIdLocalReference::new_local(String::from("object_1"));
            let _ = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_2"),
            );

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_02_eq_and_partial_eq() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            // package is more important that object

            let ref_1 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );
            let ref_2 = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_1"),
            );
            let ref_3 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_2"),
            );
            let ref_4 = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_2"),
            );

            let ref_1_bis = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );

            // Equality
            assert!(ref_1 == ref_1);
            assert!(ref_2 == ref_2);
            assert!(ref_3 == ref_3);
            assert!(ref_4 == ref_4);

            assert!(ref_1 == ref_1_bis);

            // Non-equality
            assert!(ref_1 != ref_2);
            assert!(ref_1 != ref_3);
            assert!(ref_1 != ref_4);
            assert!(ref_2 != ref_3);
            assert!(ref_2 != ref_4);
            assert!(ref_3 != ref_4);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_03_ord_and_partial_ord() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            // package is more important that object

            let ref_1 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );
            let ref_2 = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_1"),
            );
            let ref_3 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_2"),
            );
            let ref_4 = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_2"),
            );

            // Inequality
            assert!(ref_1 < ref_2);
            assert!(ref_1 < ref_3);
            assert!(ref_1 < ref_4);
            assert!(ref_2 < ref_3);
            assert!(ref_2 < ref_4);
            assert!(ref_3 < ref_4);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_04_debug() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1 = XMIIdLocalReference::new_local(String::from("object_1"));
            assert_eq!(
                format!("{:?}", ref_1),
                String::from("Uncomplete XMIIdLocalReference RefCell of \'object_1\'")
            );

            let ref_2 = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_2"),
            );
            assert_eq!(
                format!("{:?}", ref_2),
                String::from("Complete XMIIdLocalReference RefCell of \'package_2-object_2\'")
            );

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_05_set_package_id() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let mut ref_1 = XMIIdLocalReference::new_local(String::from("object_1"));
            ref_1.set_package_id(&String::from("package_1"));
            let ref_2 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );

            assert_eq!(ref_1, ref_2);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_06_get_fields() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let mut ref_1 = XMIIdLocalReference::new_local(String::from("object_1"));
            ref_1.set_package_id(&String::from("package_1"));
            let ref_2 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );

            assert_eq!(ref_1.get_object_id(), String::from("object_1"));
            assert_eq!(ref_1.get_package_id(), String::from("package_1"));
            assert_eq!(ref_2.get_object_id(), String::from("object_1"));
            assert_eq!(ref_2.get_package_id(), String::from("package_1"));

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_07_label_and_is_set() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );

            assert!(ref_1.is_set());
            assert!(ref_1.label().is_ok());
            assert_eq!(ref_1.label().unwrap(), String::from("package_1-object_1"));

            let ref_2 = XMIIdLocalReference::new_local(String::from("object_1"));

            assert!(!ref_2.is_set());
            assert!(ref_2.label().is_err());

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_01_creation() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let _: XMIIdReference<String> = XMIIdReference::new_local(String::from("object_1"));
            let _: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));
            let _: XMIIdReference<String> = XMIIdReference::default();

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_02_eq_and_partial_eq() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            // package is more important that object

            let ref_1: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));
            let ref_2: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_1"));
            let ref_3: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_2"));
            let ref_4 =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));

            let ref_1_bis =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));

            // Equality
            assert!(ref_1 == ref_1);
            assert!(ref_2 == ref_2);
            assert!(ref_3 == ref_3);
            assert!(ref_4 == ref_4);

            assert!(ref_1 == ref_1_bis);

            // Non-equality
            assert!(ref_1 != ref_2);
            assert!(ref_1 != ref_3);
            assert!(ref_1 != ref_4);
            assert!(ref_2 != ref_3);
            assert!(ref_2 != ref_4);
            assert!(ref_3 != ref_4);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_03_ord_and_partial_ord() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            // package is more important that object

            let ref_1: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));
            let ref_2: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_1"));
            let ref_3: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_2"));
            let ref_4: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));

            // Inequality
            assert!(ref_1 < ref_2);
            assert!(ref_1 < ref_3);
            assert!(ref_1 < ref_4);
            assert!(ref_2 < ref_3);
            assert!(ref_2 < ref_4);
            assert!(ref_3 < ref_4);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_04_debug() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1: XMIIdReference<String> = XMIIdReference::default();
            assert_eq!(
                format!("{:?}", ref_1),
                String::from("Empty XMIIdReference<EnumWeakCMOF> RefCell")
            );

            let ref_2: XMIIdReference<String> = XMIIdReference::new_local(String::from("object_1"));
            assert_eq!(
                format!("{:?}", ref_2),
                String::from("Uncomplete XMIIdReference<EnumWeakCMOF> RefCell of \'object_1\'")
            );

            let ref_3: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));
            assert_eq!(
                format!("{:?}", ref_3),
                String::from(
                    "UnLoaded XMIIdReference<EnumWeakCMOF> RefCell of \'package_2-object_2\'"
                )
            );

            let ref_4: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));
            let content = String::from("CONTENT !!!");
            ref_4.set_object(content)?;
            assert_eq!(
                format!("{:?}", ref_4),
                String::from(
                    "Loaded XMIIdReference<EnumWeakCMOF> RefCell of \'package_2-object_2\'"
                )
            );

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_05_set_package_id() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let mut ref_1: XMIIdReference<String> =
                XMIIdReference::new_local(String::from("object_1"));
            ref_1.set_package_id(&String::from("package_1"));
            let ref_2 =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));

            assert_eq!(ref_1, ref_2);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_06_get_fields() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let mut ref_1: XMIIdReference<String> =
                XMIIdReference::new_local(String::from("object_1"));
            ref_1.set_package_id(&String::from("package_1"));
            let ref_2: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));

            assert_eq!(ref_1.get_object_id(), String::from("object_1"));
            assert_eq!(ref_1.get_package_id(), String::from("package_1"));
            assert_eq!(ref_2.get_object_id(), String::from("object_1"));
            assert_eq!(ref_2.get_package_id(), String::from("package_1"));

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_07_set_object() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));

            assert!(!ref_1.is_set());

            let content = String::from("CONTENT !!!");
            ref_1.set_object(content)?;

            assert!(ref_1.is_set());

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_08_get_object() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));

            assert!(!ref_1.is_set());
            assert!(ref_1.get_object().is_err());

            let content = String::from("CONTENT !!!");
            ref_1.set_object(content)?;

            assert!(ref_1.is_set());

            assert_eq!(ref_1.get_object()?, String::from("CONTENT !!!"));

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_09_label() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));

            assert!(!ref_1.is_set());
            assert!(ref_1.label().is_err());

            let ref_2: XMIIdReference<String> = XMIIdReference::new_local(String::from("object_1"));

            assert!(!ref_2.is_set());
            assert!(ref_2.label().is_err());

            let ref_3: XMIIdReference<String> = XMIIdReference::new_local(String::from("object_1"));
            let content = String::from("CONTENT !!!");
            ref_3.set_object(content)?;

            assert!(!ref_3.is_set());
            assert!(ref_3.label().is_err());

            let ref_4: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));
            let content = String::from("CONTENT !!!");
            ref_4.set_object(content)?;

            assert!(ref_4.is_set());
            assert!(ref_4.label().is_ok());
            assert_eq!(ref_4.label().unwrap(), String::from("package_1-object_1"));
            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }
}
