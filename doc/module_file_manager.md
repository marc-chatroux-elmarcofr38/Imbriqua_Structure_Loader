Provide shortcut to filesystem function, with error forwarding and panic control

# How to use

Adding __FileManager__ traits to __std::fs::Path__

This traits add to __std::fs::Path__ the following function to folder :

* get_folder_content : Return the content of the folder (as ReadDir)
* create_folder : Create the folder if don't exist
* copy_folder(to) : Copy each item of a folder to a other
* move_folder(to) : Move each item of a folder to a other
* delete_folder : Delete the folder if it exist
* purge_folder : Remove all content of the folder if it exist
* check_is_dir : Panic if it's not a directory (used by other function)

This traits add to __std::fs::Path__ the following function to file :

* write_new_file : Check if the file exist, and if it's file, and if it's readable and return it (as File)
* get_file_content : Check if the file exist, and if it's file, and if it's readable and return this content (as String)
* get_file_content_as_element : Check if the file exist, and if it's file, and if it's readable and return this content (as Element)
* copy_file(to) : Copy a file to a other location
* move_file(to) : Move a file to a other location
* delete_file : Delete the file if it exist
* check_is_file : Panic if it's not a file (used by other function)

This traits add to __std::fs::Path__ the following function to all path :

* canonicalize : Canonicalize the path if it exist

## Example usecase

```rust
mod module_log;

fn main() {

    let _handle = module_log::open_logger("config_log.yml");

    error!("It's an working error log");
    warn!("It's a working warn log");
    info!("It's an working info log");
    debug!("It's a working debug log");
    trace!("It's a working trace log");

}
```

## Good practice

Use __pub mod module_file_manager__ in __main.rs__.

This practice allowing you to import traits implementation on __Path__ by using __use crate::module_file_manager::FileManager__

This practice allowing you to import __Path__ by using __use crate::module_file_manager::Path__

This practice allowing you to import __PathBuf__ by using __use crate::module_file_manager::PathBuf__

```rust
// main.rs
pub mod module_file_manager;

fn main () {
    // ...
}
```

```rust
// foo.rs
use crate::module_file_manager__::{FileManager, Path, PathBuf};

fn bar () {
    // ...

    let foo = "./foo";
    foo.check_is_dir();

    // ...
}

```

## Minimal usecase

...

## Advenced usecase

...

# Panic and failure

* PANIC_FLM01 - The 'folder' isn't a directory (or don't exist)
    * Context : __module_file_manager.rs/FileManager::check_is_dir()__
    * Info : The path isn't an existing directory
    * Cause :
        * see details in logs file to get the __Path__
        * The error come from __Path::is_dir()__ function

* PANIC_FLM02 - The 'folder' isn't readable
    * Context : __module_file_manager.rs/FileManager::get_folder_content()__
    * Info : The folder isn't readable
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __fs_extra::dir::get_dir_content2()__ function

* PANIC_FLM03 - The 'folder' can't be created
    * Context : __module_file_manager.rs/FileManager::create_folder()__
    * Info : The folder isn't creatable
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __fs_extra::dir::create_all()__ function

* PANIC_FLM04 - The 'folder' can't be copied
    * Context : __module_file_manager.rs/FileManager::copy_folder()__
    * Info : The folder isn't copied
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __fs_extra::dir::copy_items()__ function

* PANIC_FLM05 - The 'folder' can't be moved
    * Context : __module_file_manager.rs/FileManager::move_folder()__
    * Info : The folder isn't moved
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __fs_extra::dir::move_items()__ function

* PANIC_FLM06 - The 'folder' can't be deleted
    * Context : __module_file_manager.rs/FileManager::delete_folder()__
    * Info : The folder isn't deleted
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __fs_extra::dir::remove()__ function

* PANIC_FLM07 - The 'folder' can't be purged
    * Context : __module_file_manager.rs/FileManager::purge_folder()__
    * Info : The folder isn't purged
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __fs_extra::dir::remove_items()__ function

* PANIC_FLM08 - The 'file' isn't a file (or don't exist)
    * Context : __module_file_manager.rs/FileManager::check_is_file()__
    * Info : The path isn't an existing file
    * Cause :
        * see details in logs file to get the __Path__
        * The error come from __Path::is_file()__ function

* PANIC_FLM09 - The 'file' can't be created
    * Context : __module_file_manager.rs/FileManager::write_new_file()__
    * Info : The file already exist, or can't be created
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __std::fs::File::create()__ function, or __Path.exist()__

* PANIC_FLM10 - The 'file' isn't readable (as String)
    * Context : __module_file_manager.rs/FileManager::get_file_content()__
    * Info : The file isn't readed
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __fs_extra::file::read_to_string()__ function

* PANIC_FLM11 - The 'file' isn't parsable
    * Context : __module_file_manager.rs/FileManager::get_file_content_as_element()__
    * Info : The file isn't parsed
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __minidom::Element()__ parsing

* PANIC_FLM12 - The 'file' can't be copied
    * Context : __module_file_manager.rs/FileManager::copy_file()__
    * Info : The file isn't copied
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __fs_extra::file::copy()__ function

* PANIC_FLM13 - The 'file' can't be moved
    * Context : __module_file_manager.rs/FileManager::move_file()__
    * Info : The file isn't moved
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __fs_extra::file::move_file()__ function

* PANIC_FLM14 - The 'file' can't be deleted
    * Context : __module_file_manager.rs/FileManager::delete_file()__
    * Info : The file isn't deleted
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __fs_extra::file::remove()__ function

* PANIC_FLM15 - Can't canonicalize
    * Context : __module_file_manager.rs/FileManager::canonicalize()__
    * Info : The path isn't canonicalizable
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from __std::fs::canonicalize()__ function
