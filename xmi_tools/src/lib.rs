use anyhow::*;

pub mod xmi_reference;
pub use xmi_reference::*;

pub trait XMIIdentification {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error>;
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error>;
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
