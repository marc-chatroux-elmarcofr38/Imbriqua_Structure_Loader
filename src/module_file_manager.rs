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

#![warn(missing_docs)]
#![doc = include_str!("../doc/module_file_manager.md")]

// Package section
use crate::module_log::*;

// Dependencies section
pub use std::path::Path;
pub use std::path::PathBuf;

/// Provide shortcut to filesystem function, with error forwarding and panic control
pub trait FileManager {
    ///  Panic if it's not a directory
    fn check_is_dir(&self) -> ();

    ///  Return the content of the folder (as ReadDir)
    fn get_folder_content(&self) -> Vec<PathBuf>;

    /// Create the folder if don't exist
    fn create_folder(&self) -> ();

    /// Copy each item of a folder to a other
    fn copy_folder(&self, to : &Self) -> ();

    /// Move each item of a folder to a other
    fn move_folder(&self, to : &Self) -> ();

    /// Delete the folder if it exist
    fn delete_folder(&self, empty_only : bool) -> ();

    /// Remove all content of the folder if it exist
    fn purge_folder(&self) -> ();

    ///  Panic if it's not a file
    fn check_is_file(&self) -> ();

    /// Check if the file exist, and if it's file, and if it's readable and return it (as File)
    fn write_new_file(&self) -> std::fs::File;

    /// Check if the file exist, and if it's file, and if it's readable and return this content (as String)
    fn get_file_content(&self) -> String;

    /// Check if the file exist, and if it's file, and if it's readable and return this content (as Element)
    fn get_file_content_as_element(&self) -> minidom::Element;

    /// Copy a file to a other location
    fn copy_file(&self, to : &Self) -> ();

    /// Move a file to a other location
    fn move_file(&self, to : &Self) -> ();

    /// Delete the file if it exist
    fn delete_file(&self) -> ();

    /// Canonicalize the path if it exist
    fn canonicalize_pathbuf(&self) -> PathBuf;
}

impl FileManager for Path {
    ///  Panic if it's not a directory
    fn check_is_dir(&self) -> () {
        // Checking if path is dir (and if exist)
        match self.is_dir() {
            true => {
                trace!("Path {:?} is a existing directory", self);
            },
            false => {
                error!("PANIC_FLM01 - The 'folder' isn't a directory (or don't exist) : {:?}", self);
                panic!("PANIC_FLM01 - The 'folder' isn't a directory (or don't exist) : {:?}", self);
            },
        }
    }

    ///  Return the content of the folder (as ReadDir)
    fn get_folder_content(&self) -> Vec<PathBuf> {
        // Directory checking
        self.check_is_dir();
        // Setting options
        let mut options_1 = fs_extra::dir::DirOptions::new();
        options_1.depth = 1;
        // Get items
        let mut from_paths: Vec<PathBuf> = Vec::new();
        match fs_extra::dir::get_dir_content2(self, &options_1) {
            Ok(result) => {
                let to_add : Vec<String> = result.files;
                let to_add : Vec<PathBuf> = to_add.iter().map(|s| Path::new(s).to_path_buf()).collect();
                from_paths.extend(to_add);
                let to_add : Vec<String> = result.directories;
                let to_add : Vec<PathBuf> = to_add.iter().map(|s| Path::new(s).to_path_buf()).collect();
                from_paths.extend(to_add);
                trace!("Path {:?} content provided ", self);
            },
            Err(error) => {
                error!("PANIC_FLM02 - The 'folder' isn't readable : {:?} (err : {})", self, error);
                panic!("PANIC_FLM02 - The 'folder' isn't readable : {:?} (err : {})", self, error);
            },
        }
        // Remove 'self'
        from_paths.retain(|x| Path::new(&x) != self);
        // Return
        from_paths
    }

    /// Create the folder if don't exist
    fn create_folder(&self) -> ()  {
        // Exit if the folder exist
        if self.exists() {
            trace!("Folder {:?} already exist (don't create)", self);
            // Directory checking
            self.check_is_dir();
            return;
        }
        // Else, create it
        match fs_extra::dir::create_all(self, false) {
            Ok(_) => {
                trace!("Folder {:?} created", self);
            }
            Err(error) => {
                error!("PANIC_FLM03 - The 'folder' can't be created : {:?} (err : {})", self, error);
                panic!("PANIC_FLM03 - The 'folder' can't be created : {:?} (err : {})", self, error);
            }
        };
        // Directory checking
        self.check_is_dir();
    }

    /// Copy each item of a folder to a other
    fn copy_folder(&self, to : &Self) -> () {
        // Directory checking
        self.check_is_dir();
        to.create_folder();
        to.check_is_dir();
        // Setting options
        let options = fs_extra::dir::CopyOptions::new();
        // Get content
        let from_paths = self.get_folder_content();
        // Copying file
        match fs_extra::copy_items(&from_paths, to , &options) {
            Ok(_) => {
                trace!("File {:?} copied", self);
            }
            Err(error) => {
                error!("PANIC_FLM04 - The 'folder' can't be copied : {:?} (err : {})", self, error);
                panic!("PANIC_FLM04 - The 'folder' can't be copied : {:?} (err : {})", self, error);
            }
        };
    }

    /// Move each item of a folder to a other
    fn move_folder(&self, to : &Self) -> () {
        // Directory checking
        self.check_is_dir();
        to.create_folder();
        to.check_is_dir();
        // Setting options
        let options = fs_extra::dir::CopyOptions::new();
        // Get content
        let from_paths = self.get_folder_content();
        // Moving file
        match fs_extra::move_items(&from_paths, to , &options) {
            Ok(_) => {
                trace!("File {:?} moved", self);
            }
            Err(error) => {
                error!("PANIC_FLM05 - The 'folder' can't be moved : {:?} (err : {})", self, error);
                panic!("PANIC_FLM05 - The 'folder' can't be moved : {:?} (err : {})", self, error);
            }
        };
        // Delete empty folder
        self.delete_folder(true);
    }

    /// Delete the folder if it exist
    fn delete_folder(&self, empty_only : bool) -> () {
        // Directory checking
        self.check_is_dir();
        // Exit if not empty AND empty constraint
        if empty_only && (self.get_folder_content().len() != 0) {
            trace!("Folder {:?} isn't empty (don't delete)", self);
            return;
        }
        // Remove it
        match fs_extra::dir::remove(self) {
            Ok(_) => {
                trace!("Folder {:?} deleted", self);
            }
            Err(error) => {
                error!("PANIC_FLM06 - The 'folder' can't be deleted : {:?} (err : {})", self, error);
                panic!("PANIC_FLM06 - The 'folder' can't be deleted : {:?} (err : {})", self, error);
            }
        };
    }

    /// Remove all content of the folder if it exist
    fn purge_folder(&self) -> () {
        // Directory checking
        self.check_is_dir();
        // Get content
        let items = self.get_folder_content();
        // Remove each entry
        match fs_extra::remove_items(&items) {
            Ok(_) => {
                trace!("Purging content of {:?}", self);
            },
            Err(error) => {
                error!("PANIC_FLM07 - The 'folder' can't be purged : {:?} (err : {})", self, error);
                panic!("PANIC_FLM07 - The 'folder' can't be purged : {:?} (err : {})", self, error);
            },
        }
    }

    ///  Panic if it's not a file
    fn check_is_file(&self) -> () {
        // Checking if path is file (and if exist)
        match self.is_file() {
            true => {
                trace!("Path {:?} is a existing file", self);
            },
            false => {
                error!("PANIC_FLM08 - The 'file' isn't a file (or don't exist) : {:?}", self);
                panic!("PANIC_FLM08 - The 'file' isn't a file (or don't exist) : {:?}", self);
            },
        }
    }

    /// Check if the file exist, and if it's file, and if it's readable and return it (as File)
    fn write_new_file(&self) -> std::fs::File {
        // Panic if the file exist
        if self.exists() {
            error!("PANIC_FLM09 - The 'file' can't be created : {:?} (already exist)", self);
            panic!("PANIC_FLM09 - The 'file' can't be created : {:?} (already exist)", self);
        }
        // Create it
        match std::fs::File::create(self) {
            Ok(result_object) => {
                trace!("The 'file' was created : {:?}", self);
                // File checking
                self.check_is_file();
                result_object
            }
            Err(error) => {
                error!("PANIC_FLM09 - The 'file' can't be created : {:?} (err : {})", self, error);
                panic!("PANIC_FLM09 - The 'file' can't be created : {:?} (err : {})", self, error);
            }
        }
    }

    /// Check if the file exist, and if it's file, and if it's readable and return this content (as String)
    fn get_file_content(&self) -> String {
        // File checking
        self.check_is_file();
        // Get the content
        match fs_extra::file::read_to_string(self) {
            Ok(result_object) => {
                trace!("The 'file' is readable : {:?}", self);
                result_object
            }
            Err(error) => {
                error!("PANIC_FLM10 - The 'file' isn't readable (as String) : {:?} (err : {})", self, error);
                panic!("PANIC_FLM10 - The 'file' isn't readable (as String) : {:?} (err : {})", self, error);
            }
        }
    }

    /// Check if the file exist, and if it's file, and if it's readable and return this content (as Element)
    fn get_file_content_as_element(&self) -> minidom::Element {
        // File checking
        self.check_is_file();
        // Get the content
        let content = self.get_file_content();
        // Parsing file content to Element object class
        let element_file : minidom::Element = match content.parse() {
            Ok(result_object) => {
                trace!("Parsing Element : {:?}", self);
                result_object
            }
            Err(error) => {
                error!("PANIC_FLM11 - The 'file' isn't parsable : {:?} (err : {})", self, error);
                panic!("PANIC_FLM11 - The 'file' isn't parsable : {:?} (err : {})", self, error);
            }
        };
        // Return result
        element_file
    }

    /// Copy a file to a other location
    fn copy_file(&self, to : &Self) -> () {
        // File checking
        self.check_is_file();
        // Configuration
        let options = fs_extra::file::CopyOptions::new();
        // Copying file
        match fs_extra::file::copy(self, to , &options) {
            Ok(_) => {
                trace!("File {:?} copied", self);
            }
            Err(error) => {
                error!("PANIC_FLM12 - The 'file' can't be copied : {:?} (err : {})", self, error);
                panic!("PANIC_FLM12 - The 'file' can't be copied : {:?} (err : {})", self, error);
            }
        };
    }

    /// Move a file to a other location
    fn move_file(&self, to : &Self) -> () {
        // File checking
        self.check_is_file();
        // Configuration
        let options = fs_extra::file::CopyOptions::new();
        // Moving file
        match fs_extra::file::move_file(self, to , &options) {
            Ok(_) => {
                trace!("File {:?} moved", self);
            }
            Err(error) => {
                error!("PANIC_FLM13 - The 'file' can't be moved : {:?} (err : {})", self, error);
                panic!("PANIC_FLM13 - The 'file' can't be moved : {:?} (err : {})", self, error);
            }
        };
    }

    /// Delete the file if it exist
    fn delete_file(&self) -> () {
        // File checking
        self.check_is_file();
        // Delete file
        match fs_extra::file::remove(self) {
            Ok(_) => {
                trace!("File {:?} deleted", self);
            }
            Err(error) => {
                error!("PANIC_FLM14 - The 'file' can't be deleted : {:?} (err : {})", self, error);
                panic!("PANIC_FLM14 - The 'file' can't be deleted : {:?} (err : {})", self, error);
            }
        };
    }

    /// Canonicalize the path if it exist
    fn canonicalize_pathbuf(&self) -> PathBuf {
        // Canonicalize
        match std::fs::canonicalize(self) {
            Ok(result) => {
                info!("Can canonicalize {:?} to {:?}", self, result);
                result
            },
            Err(error) => {
                error!("PANIC_FLM15 - Can't canonicalize {:?} - {}", self, error);
                panic!("PANIC_FLM15 - Can't canonicalize {:?} - {}", self, error);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::*;
    use crate::module_log::tests::initialize_log_for_test;

    #[test]
    fn module_flm_01_check_is_dir() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder = Path::new("tests/module_file_manager/module_flm_01_check_is_dir/folder_to_check");
        // Preparing
        // Test
        folder.check_is_dir();
    }

    #[test]
    #[should_panic(expected = "PANIC_FLM01")]
    fn module_flm_01_check_is_dir_panic_01() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder = Path::new("tests/module_file_manager/module_flm_01_check_is_dir_panic_01/folder_to_check");
        // Preparing
        // Test
        folder.check_is_dir();
    }

    #[test]
    fn module_flm_02_get_folder_content() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder_1 = Path::new("tests/module_file_manager/module_flm_02_get_folder_content");
        let folder_2 = Path::new("tests/module_file_manager/module_flm_02_get_folder_content/random_folder_01");
        let folder_3 = Path::new("tests/module_file_manager/module_flm_02_get_folder_content/random_folder_01/random_folder_03");
        let folder_4 = Path::new("tests/module_file_manager/module_flm_02_get_folder_content/random_folder_02");
        // Preparing
        // Test
        assert_eq!(folder_1.get_folder_content().len(), 3);
        assert_eq!(folder_2.get_folder_content().len(), 2);
        assert_eq!(folder_3.get_folder_content().len(), 1);
        assert_eq!(folder_4.get_folder_content().len(), 1);
    }

    #[test]
    fn module_flm_03_create_folder() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder = Path::new("tests/module_file_manager/module_flm_03_create_folder/folder_to_create");
        // Preparing
        if folder.exists() {folder.delete_folder(false);}
        assert_eq!(folder.exists(), false);
        // Test
        folder.create_folder();
    }

    #[test]
    fn module_flm_04_copy_folder() {
        // Logs
        initialize_log_for_test();
        // Setting
        let from = Path::new("tests/module_file_manager/module_flm_04_copy_folder/from");
        let to = Path::new("tests/module_file_manager/module_flm_04_copy_folder/to");
        // Preparing
        if to.exists() {to.delete_folder(false);}
        assert_eq!(to.exists(), false);
        // Test
        from.copy_folder(to);
        let to_checking = Path::new("tests/module_file_manager/module_flm_04_copy_folder/to");
        assert_eq!(to_checking.get_folder_content().len(), 3);
        let to_checking = Path::new("tests/module_file_manager/module_flm_04_copy_folder/to/random_folder_01");
        assert_eq!(to_checking.get_folder_content().len(), 2);
        let to_checking = Path::new("tests/module_file_manager/module_flm_04_copy_folder/to/random_folder_01/random_folder_03");
        assert_eq!(to_checking.get_folder_content().len(), 1);
        let to_checking = Path::new("tests/module_file_manager/module_flm_04_copy_folder/to/random_folder_02");
        assert_eq!(to_checking.get_folder_content().len(), 1);
    }

    #[test]
    fn module_flm_05_move_folder() {
        // Logs
        initialize_log_for_test();
        // Setting
        let from_template = Path::new("tests/module_file_manager/module_flm_05_move_folder/from_template");
        let from = Path::new("tests/module_file_manager/module_flm_05_move_folder/from");
        let to = Path::new("tests/module_file_manager/module_flm_05_move_folder/to");
        // Preparing
        if from.exists() {from.delete_folder(false);}
        assert_eq!(from.exists(), false);
        if to.exists() {to.delete_folder(false);}
        assert_eq!(to.exists(), false);
        from_template.copy_folder(from);
        let from_checking = Path::new("tests/module_file_manager/module_flm_05_move_folder/from");
        assert_eq!(from_checking.get_folder_content().len(), 3);
        let from_checking = Path::new("tests/module_file_manager/module_flm_05_move_folder/from/random_folder_01");
        assert_eq!(from_checking.get_folder_content().len(), 2);
        let from_checking = Path::new("tests/module_file_manager/module_flm_05_move_folder/from/random_folder_01/random_folder_03");
        assert_eq!(from_checking.get_folder_content().len(), 1);
        let from_checking = Path::new("tests/module_file_manager/module_flm_05_move_folder/from/random_folder_02");
        assert_eq!(from_checking.get_folder_content().len(), 1);
        // Test
        from.move_folder(to);
        assert_eq!(from.exists(), false);
        let to_checking = Path::new("tests/module_file_manager/module_flm_05_move_folder/to");
        assert_eq!(to_checking.get_folder_content().len(), 3);
        let to_checking = Path::new("tests/module_file_manager/module_flm_05_move_folder/to/random_folder_01");
        assert_eq!(to_checking.get_folder_content().len(), 2);
        let to_checking = Path::new("tests/module_file_manager/module_flm_05_move_folder/to/random_folder_01/random_folder_03");
        assert_eq!(to_checking.get_folder_content().len(), 1);
        let to_checking = Path::new("tests/module_file_manager/module_flm_05_move_folder/to/random_folder_02");
        assert_eq!(to_checking.get_folder_content().len(), 1);
    }

    #[test]
    fn module_flm_06_delete_folder() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder_with_content_to_remove = Path::new("tests/module_file_manager/module_flm_06_delete_folder/empty_folder_to_delete");
        let from = Path::new("tests/module_file_manager/module_flm_06_delete_folder/from");
        let folder_with_content_to_not_remove = Path::new("tests/module_file_manager/module_flm_06_delete_folder/not_empty_folder_to_delete");
        // Preparing
        if !folder_with_content_to_remove.exists() {from.copy_folder(folder_with_content_to_remove);}
        assert_eq!(folder_with_content_to_remove.exists(), true);
        if !folder_with_content_to_not_remove.exists() {from.copy_folder(folder_with_content_to_not_remove);}
        assert_eq!(folder_with_content_to_not_remove.exists(), true);
        // Test
        folder_with_content_to_remove.delete_folder(false);
        assert_eq!(folder_with_content_to_remove.exists(), false);
        folder_with_content_to_not_remove.delete_folder(true);
        assert_eq!(folder_with_content_to_not_remove.exists(), true);
    }

    #[test]
    fn module_flm_07_purge_folder() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder_with_content_to_purge = Path::new("tests/module_file_manager/module_flm_07_purge_folder/not_empty_folder_to_purge");
        let from = Path::new("tests/module_file_manager/module_flm_07_purge_folder/from");
        // Preparing
        if !folder_with_content_to_purge.exists() {from.copy_folder(folder_with_content_to_purge);}
        assert_eq!(folder_with_content_to_purge.exists(), true);
        if folder_with_content_to_purge.get_folder_content().len() != 3 {from.copy_folder(folder_with_content_to_purge);}
        assert_eq!(folder_with_content_to_purge.get_folder_content().len(), 3);
        // Test
        folder_with_content_to_purge.purge_folder();
        assert_eq!(folder_with_content_to_purge.exists(), true);
        assert_eq!(folder_with_content_to_purge.get_folder_content().len(), 0);
    }

    #[test]
    fn module_flm_08_check_is_file() {
        // Logs
        initialize_log_for_test();
        // Setting
        let file = Path::new("tests/module_file_manager/module_flm_08_check_is_file/file_to_check");
        // Preparing
        // Test
        file.check_is_file();
    }

    #[test]
    #[should_panic(expected = "PANIC_FLM08")]
    fn module_flm_08_check_is_file_panic_01() {
        // Logs
        initialize_log_for_test();
        // Setting
        let file = Path::new("tests/module_file_manager/module_flm_08_check_is_file_panic_01/file_to_check");
        // Preparing
        // Test
        file.check_is_file();
    }

    #[test]
    fn module_flm_09_write_new_file() {
        // Logs
        initialize_log_for_test();
        // Setting
        let file = Path::new("tests/module_file_manager/module_flm_09_write_new_file/writing_file.txt");
        // Preparing
        if file.exists() {file.delete_file();}
        assert_eq!(file.exists(), false);
        // Test
        let mut writing_file= file.write_new_file();
        assert!(write!(writing_file, "AAA AAA AAA").is_ok());
    }

    #[test]
    fn module_flm_10_get_file_content() {
        // Logs
        initialize_log_for_test();
        // Setting
        let file = Path::new("tests/module_file_manager/module_flm_10_get_file_content/file_to_read.txt");
        // Preparing
        // Test
        let reading_file= file.get_file_content();
        assert_eq!(reading_file, String::from("AAA AAA AAA"));
    }

    #[test]
    fn module_flm_11_get_file_content_as_element() {
        // Logs
        initialize_log_for_test();
        // Setting
        let file = Path::new("tests/module_file_manager/module_flm_11_get_file_content_as_element/file_to_read.txt");
        // Preparing
        // Test
        let reading_file= file.get_file_content_as_element();
        assert_eq!(reading_file, "<elem xmlns=\"ns1\" a=\"b\" />".parse().unwrap());
    }

    #[test]
    fn module_flm_12_copy_file() {
        // Logs
        initialize_log_for_test();
        // Setting
        let from = Path::new("tests/module_file_manager/module_flm_12_copy_file/from_file.txt");
        let to = Path::new("tests/module_file_manager/module_flm_12_copy_file/to_file.txt");
        // Preparing
        if to.exists() {to.delete_file();}
        assert_eq!(to.exists(), false);
        // Test
        from.copy_file(to);
        assert_eq!(to.get_file_content(), String::from("AAA AAA AAA"));
    }

    #[test]
    fn module_flm_13_move_file() {
        // Logs
        initialize_log_for_test();
        // Setting
        let from_template = Path::new("tests/module_file_manager/module_flm_13_move_file/from_template_file.txt");
        let from = Path::new("tests/module_file_manager/module_flm_13_move_file/from_file.txt");
        let to = Path::new("tests/module_file_manager/module_flm_13_move_file/to_file.txt");
        // Preparing
        if from.exists() {from.delete_file();}
        assert_eq!(from.exists(), false);
        if to.exists() {to.delete_file();}
        assert_eq!(to.exists(), false);
        from_template.copy_file(from);
        assert_eq!(from.get_file_content(), String::from("AAA AAA AAA"));
        // Test
        from.move_file(to);
        assert_eq!(from.exists(), false);
        assert_eq!(to.get_file_content(), String::from("AAA AAA AAA"));
    }

    #[test]
    fn module_flm_14_delete_file() {
        // Logs
        initialize_log_for_test();
        // Setting
        let file_template = Path::new("tests/module_file_manager/module_flm_14_delete_file/template_file_to_delete.txt");
        let file_to_delete = Path::new("tests/module_file_manager/module_flm_14_delete_file/file_to_delete.txt");
        // Preparing
        if !file_to_delete.exists() {file_template.copy_file(file_to_delete);}
        assert_eq!(file_to_delete.exists(), true);
        assert_eq!(file_to_delete.get_file_content(), String::from("AAA AAA AAA"));
        // Test
        file_to_delete.delete_file();
        assert_eq!(file_to_delete.exists(), false);
    }

    #[test]
    fn module_flm_15_canonicalize_pathbuf() {
        // Logs
        initialize_log_for_test();
        // Setting
        let file = Path::new("tests/module_file_manager/module_flm_15_canonicalize_pathbuf/file_to_canonicalize.txt");
        let folder = Path::new("tests/module_file_manager/module_flm_15_canonicalize_pathbuf/folder_to_canonicalize");
        // Preparing
        // Test
        let _ = file.canonicalize_pathbuf(); // Just don't panic
        let _ = folder.canonicalize_pathbuf(); // Just don't panic
    }

    #[ignore]
    fn _test_template() {
        // Logs
        initialize_log_for_test();
        // Setting
        // Preparing
        // Test
    }
}
