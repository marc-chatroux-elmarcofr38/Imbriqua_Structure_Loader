# WIKI : How Imbriqua Structure work ?



## Main



## Logs



## List of 



## List of Panics

* PANIC_LOG01 - Error during the loading on logs modules
    * Context : __module_log.rs/open_modules()__
    * Info : No logs are provided, so, make panic
    * Cause : The error come from __load_configuration_backup()__ function or __get_backup_config()__

* PANIC_FILE01 - A folder can't be created
    * Context : __module_dependencies_explorer.rs/check_folder_exist()__
    * Panic action caused by __ERROR_FILE01__
    * See __ERROR_FILE01__ information

* PANIC_FILE02 - A folder isn't readable
    * Context : __module_dependencies_explorer.rs/check_read_path()__
    * Panic action caused by __ERROR_FILE02__
    * See __ERROR_FILE02__ information

* PANIC_FILE03 - A file don't exist
    * Context : __module_dependencies_explorer.rs/check_file_exist()__
    * Panic action caused by __ERROR_FILE03__
    * See __ERROR_FILE03__ information

* PANIC_FILE04 - A file isn't readable
    * Context : __module_dependencies_explorer.rs/check_read_file()__
    * Panic action caused by __ERROR_FILE04__
    * See __ERROR_FILE04__ information

* PANIC_FILE05 - A file isn't parsable
    * Context : __module_dependencies_explorer.rs/get_package_from_path()__
    * Panic action caused by __ERROR_FILE05__
    * See __ERROR_FILE05__ information

* PANIC_FILE06 - CMOF file don't contain the needed package
    * Context : __module_dependencies_explorer.rs/get_package_from_path()__
    * Panic action caused by __ERROR_FILE06__
    * See __ERROR_FILE06__ information

* PANIC_FILE07 - Unloaded dependencies : suspicious of circular dependencies
    * Context : __module_dependencies_explorer.rs/LoadingTracker::import_dependencies_file()__
    * Panic action caused by __ERROR_FILE07__
    * See __ERROR_FILE07__ information

* PANIC_FILE08 - __packageImport__ element without __importedPackage__ child
    * Context : __module_dependencies_explorer.rs/LoadingTracker::add_dependencies()__
    * Panic action caused by __ERROR_FILE08__
    * See __ERROR_FILE08__ information

* PANIC_FILE09 - __importedPackage__ element without __href__ attribute
    * Context : __module_dependencies_explorer.rs/LoadingTracker::add_dependencies()__
    * Panic action caused by __ERROR_FILE09__
    * See __ERROR_FILE09__ information

* PANIC_FILE10 - __href__ attribute without '#' separator
    * Context : __module_dependencies_explorer.rs/LoadingTracker::add_dependencies()__
    * Panic action caused by __ERROR_FILE10__
    * See __ERROR_FILE10__ information

 


## List of Errors

* ERROR_FILE01 - A folder can't be created
    * Context : __module_dependencies_explorer.rs/check_folder_exist()__
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of __std::fs::create_dir()__

* ERROR_FILE02 - A folder can't be readed
    * Context : __module_dependencies_explorer.rs/check_read_path()__
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of __std::path::Path::read_dir()__

* ERROR_FILE03 - A file don't exist
    * Context : __module_dependencies_explorer.rs/check_file_exist()__
    * Cause : see details in logs file to get :
        * Value of file path

* ERROR_FILE04 - A file can't be readed
    * Context : __module_dependencies_explorer.rs/check_read_file()__
    * Cause : see details in logs file to get :
        * Value of file path
        * error informations of __std::fs::read_to_string()__

* ERROR_FILE05 - A file isn't parsable
    * Context : __module_dependencies_explorer.rs/get_package_from_path()__
    * Cause : see details in logs file to get :
        * Value of file path
        * error informations of Element parsing

* ERROR_FILE06 - CMOF file don't contain the needed package
    * Context : __module_dependencies_explorer.rs/get_package_from_path()__
    * Cause : see details in logs file to get :
        * file name
        * readed package name

* ERROR_FILE07 - Unloaded dependencies : suspicious of circular dependencies
    * Context : __module_dependencies_explorer.rs/LoadingTracker::import_dependencies_file()__
    * Cause : a dependencies of the file was previously reserved (circular exploration)
    * Cause : see details in logs file to get :
        * main package
        * dependencies package

* ERROR_FILE08 - __packageImport__ element without __importedPackage__ child
    * Context : __module_dependencies_explorer.rs/LoadingTracker::import_dependencies_file()__
    * Cause : see details in logs file to get :
        * name of package with error

* ERROR_FILE09 - __importedPackage__ element without __href__ attribute
    * Context : __module_dependencies_explorer.rs/LoadingTracker::import_dependencies_file()__
    * Cause : see details in logs file to get :
        * name of package with error



* PANIC_FILE10 - __href__ attribute without '#' separator
    * Context : __module_dependencies_explorer.rs/LoadingTracker::add_dependencies()__
    * Panic action caused by __ERROR_FILE10__
    * See __ERROR_FILE10__ information

* ERROR_FILE10 - __href__ attribute without '#' separator
    * Context : __module_dependencies_explorer.rs/LoadingTracker::import_dependencies_file()__
    * Cause : a dependencies of the file was previously reserved (circular exploration)
    * Cause : see details in logs file to get :
        * main package
        * dependencies package




* ERROR_FILE50 - Error during removing of a empty folder
    * Context : __module_dependencies_explorer.rs/check_remove_dir()__
    * Cause : see details in logs file to get :
        * Value of file path
        * error informations of __std::fs::remove_dir()__




## List of Warnings

* WARN_LOG01 - Error during default configuration loading
    * Context : __module_log.rs/open_modules()__
    * Info : The file __config_log.yml__ can't be loaded in log4rs configuration
    * Info : A backup logging configuration may be load,logs are in __imbriqua_structure.log__ file
    * Cause : See logs for syntaxe error details, or deserialize error details
