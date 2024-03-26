extern crate minidom;

use std::path::Path;
use std::collections::HashMap;
use std::fs::{create_dir, read_to_string, remove_dir, ReadDir};
use chrono::Local;
use log::{debug, error, info, trace, warn};
use std::fmt;
use minidom::Element;

#[derive(Clone, PartialEq, Debug)]
pub struct FileEnv {
    pub input_folder : String,
    output_folder : String,
    pub output_subfolder : String,
}

impl FileEnv {
    pub fn new() -> Self {

        //Set input folder path and output subfolder path
        let str_input_folder : String = String::from("metamodel_file/");
        let str_output_folder : String = String::from("output_file/");
        let time_string : String = Local::now().format("%Y-%m-%d_%Hh%Mm%S/").to_string();

        // Create instance
        let result = FileEnv {
            input_folder : str_input_folder.clone(),
            output_folder : str_output_folder.clone(),
            output_subfolder : str_output_folder.clone() + time_string.as_str(),
            // dependencies : Vec::new(),
        };
        
        // Checking instance
        check_folder_exist(&result.input_folder);
        check_read_path(&result.input_folder);
        check_folder_exist(&result.output_folder);
        check_read_path(&result.output_folder);
        check_folder_exist(&result.output_subfolder);
        check_read_path(&result.output_subfolder);

        // Return result
        result
    }

    pub fn delete_if_empty (&self) {
        /*
            Process function for removing output folder if no result
        */
    
        if check_remove_dir(&self.output_subfolder.as_str()) {
                info!("FileEnv : folder \"{}\" deleted (because is empty)", self.output_subfolder);
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum LoadingState {
    Empty,      // No Element
    Loaded,     // With Element
    Finished,   // Element converted
}

#[derive(Clone, PartialEq, Debug)]
struct LoadingPackage {
    filename : String,
    id : String,
    object : Element,
    state : LoadingState,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LoadingTracker {
    file_env : FileEnv,
    loaded_package : HashMap<String, LoadingPackage>,
    importing_order : HashMap<String, usize>,
}

impl LoadingTracker {
    pub fn new() -> Self {

        // Create instance
        let result = LoadingTracker {
            file_env : FileEnv::new(),
            loaded_package : HashMap::new(),
            importing_order : HashMap::new(),
        };

        // Return result
        result
    }

    pub fn import_dependencies_file(&mut self, main_file : &str, package_id : &str, parent_label : &str) {
        /*
            Load minidom element from a gived package, including dependencies
            Save element in loaded_package
    
            Input :
             - main_file (&str) : file to load in self.input_folder
             - package_id (&str) : package name wanted in main_file
             - parent_label (&str) : parent package label, used in logs command
    
            Error :
             - none
        */

        // Define hashmap key
        let mut label = String::from(main_file);
        label.push_str("#");
        label.push_str(package_id);

        // Check if the loading is necessary
        if self.is_package_already_loaded(label.clone()) && self.loaded_package.get_key_value(&label.clone()).unwrap().1.state == LoadingState::Empty {
            error!("ERROR_FILE07 - Unloaded dependencies : suspicious of circular dependencies ({a} importing {b})", a=label.clone(), b=parent_label);
            panic!("PANIC_FILE07 - Unloaded dependencies : suspicious of circular dependencies ({a} importing {b})", a=label.clone(), b=parent_label);
        }else if self.is_package_already_loaded(label.clone()) {
            info!("Loading \"{}\" : NOPE : already loaded", label.clone());
            return
        } else {
            info!("Loading \"{}\" : START", label.clone());
        }

        // Add empty element entry in loaded_package (prevent circular loading)
        self.add_empty_package(main_file, package_id, label.clone());
    
        // Generate file path
        let mut file_path = self.file_env.input_folder.clone();
        file_path.push_str(main_file);

        // Load package element
        let package_element = get_package_from_path(file_path.as_str(), package_id);

        // Evaluate dependencies, and load it
        self.add_dependencies(package_element.clone(), label.clone());
        
        // Add package element in loaded_package
        self.add_package(package_element.clone(), main_file, package_id, label.clone());
        
        // End logs
        info!("Loading \"{}\" : Finished", label.clone());
    }

    fn add_empty_package(&mut self, file : &str, package : &str, label : String) {
        /*
            Save minimal LoadingPackage object in loaded_package
            Set "state" to empty to prevent circular dependencies loading (dependencies has loaded before changing "state")
    
            Input :
             - file (&str) : file to load in self.input_folder
             - package (&str) : package name wanted in main_file
             - label (String) : label of the package
    
            Error :
             - none
        */

        // Create minimal LoadingPackage
        let package_object = LoadingPackage {
            filename : String::from(file),
            id : String::from(package),
            object : Element::builder("", "").build(),
            state : LoadingState::Empty,
        };

        // Save object in hashmap attribute
        self.loaded_package.insert(label, package_object);
    }

    fn add_dependencies(&mut self, element : Element, label : String) {
        /*
            Get dependencies of a Element (UML XMI notation)
            And import dependencies
    
            Input :
             - element (Element) : Minidom ELement object to analyse
             - label (String) : package label
    
            Error :
             - none
        */

        for child in element.children() {
            if child.is("packageImport", "") {
                // Go to "importedPackage" child
                let imported_package = match child.get_child("importedPackage", "") {
                    Some(result_object) => {
                        result_object
                    },
                    None => {
                        error!("ERROR_FILE08 - packageImport element without importedPackage child : package = \"{}\"", label);
                        panic!("PANIC_FILE08 - packageImport element without importedPackage child : package = \"{}\"", label);
                    },
                };

                // Get "href" attribute
                let package_to_import = match imported_package.attr("href") {
                    Some(result_object) => {
                        result_object
                    },
                    None => {
                        error!("ERROR_FILE09 - importedPackage element without href attribute : package = \"{}\"", label);
                        panic!("PANIC_FILE09 - importedPackage element without href attribute : package = \"{}\"", label);
                    },
                };

                //
                match package_to_import.find('#') {
                    Some(split_index) => {
                        info!("Loading \"{}\" : need to load \"{}\"", label.clone(), package_to_import);
                        let package_file : String = package_to_import[..split_index].to_string();
                        let split_index = split_index + 1;
                        let package_id : String = package_to_import[split_index..].to_string();
                        self.import_dependencies_file(package_file.as_str(), package_id.as_str(), label.clone().as_str());

                    }
                    None => {
                        error!("ERROR_FILE10 - href attribute without '#' separator : package = \"{}\", href = \"{}\"", label, package_to_import);
                        panic!("PANIC_FILE10 - href attribute without '#' separator : package = \"{}\", href = \"{}\"", label, package_to_import);
                    }
                }
            };
        };
    }

    fn add_package(&mut self, element : Element, file : &str, package : &str, label : String) {
        /*
            Save complete LoadingPackage object in loaded_package
    
            Input :
            - element (minidom::element::Element) : Element to save
            - file (&str) : file to load in self.input_folder
            - package (&str) : package name wanted in main_file
    
            Error :
            - none
        */

        // Create full LoadingPackage
        let package_object = LoadingPackage {
            filename : String::from(file),
            id : String::from(package),
            object : element,
            state : LoadingState::Loaded,
        };

        // Save object in hashmap attribute
        self.loaded_package.insert(label.clone(), package_object);

        // Define treatment order
        self.importing_order.insert(label, self.importing_order.len() + 1);
    }

    fn is_package_already_loaded(&self, label : String) -> bool {

                                            // Check if the key is already used
                                            self.loaded_package.contains_key(&label)
    }

    pub fn close(&self) {
        self.file_env.delete_if_empty();
    }
}

impl fmt::Display for LoadingTracker {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result : String = String::new();
        result.push_str("---- LoadingTracker ---");
        result.push_str("\n");
        result.push_str("\n file_env : ");
        result.push_str(format!("{:#?}", &self.file_env).as_str());
        result.push_str("\n importing_order : ");
        result.push_str(format!("{:#?}", &self.importing_order).as_str());
        result.push_str("\n");
        write!(f, "{}", result.as_str())
    }
}

impl LoadingTracker {
    pub fn prebuild(&self) {
        
    }
}





// #############################################################
//
//    TOOLS FUNCTIONS
//
// #############################################################

fn check_folder_exist(file_path_str : &str) -> () {
    /*
        Make sure that a folder exist
        Check if the folder exist, else, try to create it

        Input :
         - file_path_str (&str) : dir path to create ifdon't exist

        Error :
         - jumping error of std::fs::create_dir
    */

    // Exit if the folder exist
    match Path::new(&file_path_str).exists() {
        false => {
            trace!("CheckFile : Folder \"{}\" don't exist", &file_path_str);
        },
        true => {
            trace!("CheckFile : Folder \"{}\" exist", &file_path_str);
            return ();
        },
    };

    // Create the dir
    match create_dir(&file_path_str) {
        Ok(_) => {
            trace!("CheckFile : Output subfolder created \"{}\"", &file_path_str);
        }
        Err(err_object) => {
            error!("ERROR_FILE01 - A folder can't be created - \"{}\" : {}", &file_path_str, err_object);
            panic!("PANIC_FILE01 - A folder can't be created - \"{}\" : {}", &file_path_str, err_object);
        }
    };
}

fn check_read_path(file_path_str : &str) -> ReadDir{
    /*
        Check if a folder is readable

        Input :
         - file_path_str (&str) : dir path to check

        Error :
         - jumping error of std::path::Path::read_dir
    */

    // Check if the folder exist
    check_folder_exist(file_path_str);

    // Check if the folder is readable
    match Path::new(file_path_str).read_dir() {
        Ok(result_object) => {
            trace!("CheckFile : Folder is readable \"{}\"", file_path_str);
            result_object
        }
        Err(err_object) => {
            error!("ERROR_FILE02 - A folder isn't readable - \"{}\" : {}", &file_path_str, err_object);
            panic!("PANIC_FILE02 - A folder isn't readable - \"{}\" : {}", &file_path_str, err_object);
        }
    }
}

fn check_file_exist(file_path_str : &str) -> () {
    /*
        Check if the file exist

        Input :
         - file_path_str (&str) : file path to check

        Error :
         - none
    */

    // Exit if the folder exist
    match Path::new(&file_path_str).exists() {
        true => {
            trace!("CheckFile : Folder \"{}\" exist", &file_path_str);
        },
        false => {
            error!("ERROR_FILE03 - A file don't exist - \"{}\"", &file_path_str);
            panic!("PANIC_FILE03 - A file don't exist - \"{}\"", &file_path_str);
        },
    };
}

fn check_read_file(file_path_str : &str) -> String {
    /*
        Check if the file is readable

        Input :
         - file_path_str (&str) : file path to check

        Error :
         - jumping error of std::fs::read_to_string
    */

    // Check if the file exist
    check_file_exist(file_path_str);

    // Check if the file is readable 
    match read_to_string(file_path_str) {
        Ok(result_object) => {
            trace!("CheckFile : File is readable \"{}\"", file_path_str);
            result_object
        }
        Err(err_object) => {
            error!("ERROR_FILE04 - A file isn't readable - \"{}\" : {}", &file_path_str, err_object);
            panic!("PANIC_FILE04 - A file isn't readable - \"{}\" : {}", &file_path_str, err_object);
        }
    }
}

fn check_remove_dir(file_path_str : &str) -> bool {
    /*
        Check if the folder is deletable (empty)

        Input :
         - file_path_str (&str) : file path to delete

        Output :
         - true if removed, else false

        Error :
         - jumping error of std::fs::remove_dir
    */

        // Exit if not empty
        if !check_read_path(file_path_str).next().is_none() {
            trace!("CheckFile : Output subfolder isn't empty \"{}\"", file_path_str);
            return false
        } else {
            trace!("CheckFile : Output subfolder is empty \"{}\"", file_path_str);
        };

        match remove_dir(file_path_str) {
            Ok(_) => {
                return true
            },
            Err(error) => {
                error!("ERROR_FILE50 - Error during removing of a empty folder - \"{}\" : {}", file_path_str, error);
                return false
            },
        }
}

fn get_package_from_path(file_path_str : &str, package_id : &str) -> Element {
    /*
        Return the minidom element stored in the input file path

        Input :
         - file_path_str (&str) : file path to read

        Error :
         - jump error of check_read_file
         - jump error of Element parsing
    */
    
    // Check if the file is readable
    let file = check_read_file(file_path_str);

    // Parsing file content to Element object class
    let element_file : Element = match file.parse() {
        Ok(result_object) => {
            trace!("FileEnv : Reading Element from \"{}\"", file_path_str);
            result_object
        }
        Err(err_object) => {
            error!("ERROR_FILE05 - \"{}\" : {}", &file_path_str, err_object);
            panic!("PANIC_FILE05 - A file isn't parsable - \"{}\" : {}", &file_path_str, err_object);}
    };

    // Find "package_id" child
    for child in element_file.children() {
        if child.is("Package", "http://schema.omg.org/spec/MOF/2.0/cmof.xml") {
            if child.attr("xmi:id") == Some(package_id) {
                return child.clone()
            }
        }
    };

    error!("ERROR_FILE06 - file name = \"{}\", package name = \"{}\"", &file_path_str, package_id);
    panic!("PANIC_FILE06 - CMOF file don't contain the needed package - file name = \"{}\", package name = \"{}\"", &file_path_str, package_id);
}
