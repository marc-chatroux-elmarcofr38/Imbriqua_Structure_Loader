use anyhow::*;

pub mod xmi_reference;
pub use xmi_reference::*;

pub trait XMIIdentity: XMIIdentification + PartialEq + Eq + PartialOrd + Ord {}

pub trait XMIIdentification {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error>;
    fn get_xmi_id(&self) -> &XMIIdLocalReference;
}

// /// Allow to finish XMIId of object and collect all CMOF object
// /// Use "dict_setting" for share content between parent object to child object
// /// Use "dict_object" for collect all object
// fn new_collect_object<T>(
//     &mut self,
//     dict_setting: &mut BTreeMap<String, String>,
//     dict_object: &mut BTreeMap<String, T>,
// ) -> Result<(), anyhow::Error>;
// /// Allow to define the post-treatment method : post_deserialize
// /// Link external XMI Id of object by matching it on "dict_object"
// /// Use "dict_object" for obtain object between objects
// fn new_make_post_deserialize<T>(
//     &self,
//     dict_object: &mut BTreeMap<String, T>,
// ) -> Result<(), anyhow::Error>;

// // ####################################################################################################
// //
// // ####################################################################################################

// impl PartialEq for CMOFAssociation {
//     fn eq(&self, other: &Self) -> bool {
//         self.xmi_id == other.xmi_id
//     }
// }

// impl Eq for CMOFAssociation {}

// impl PartialOrd for CMOFAssociation {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for CMOFAssociation {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.xmi_id.cmp(&other.xmi_id)
//     }
// }
