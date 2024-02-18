# WIKI : How Imbriqua Structure work ?



## Main



## Logs



## List of 



## List of Panics

* PANIC_LOG01 - Error during the loading on logs modules
    * Context : __module_log.rs/open_modules()__
    * Info : No logs are provided, so, make panic
    * Cause : The error come from __load_configuration_backup()__ function or __get_backup_config()__
        
* PANIC_FILE01 - Input main folder can't be created
    * Panic action caused by __ERROR_FILE01__
    * See __ERROR_FILE01__ information
        
* PANIC_FILE02 - Input main folder isn't readable
    * Panic action caused by __ERROR_FILE02__
    * See __ERROR_FILE02__ information
        
* PANIC_FILE03 - Output main folder can't be created
    * Panic action caused by __ERROR_FILE01__
    * See __ERROR_FILE01__ information
        
* PANIC_FILE04 - Output main folder isn't readable
    * Panic action caused by __ERROR_FILE02__
    * See __ERROR_FILE02__ information
        
* PANIC_FILE05 - Output subfolder can't be created
    * Panic action caused by __ERROR_FILE01__
    * See __ERROR_FILE01__ information
        
* PANIC_FILE06 - Output subfolder isn't readable
    * Panic action caused by __ERROR_FILE02__
    * See __ERROR_FILE02__ information
        
* PANIC_FILE07 - A CMOF dependencies doesn't exist
    * Panic action caused by __ERROR_FILE04__
    * See __ERROR_FILE04__ information
        
* PANIC_FILE08 - A CMOF dependencies isn't readable
    * Panic action caused by __ERROR_FILE05__
    * See __ERROR_FILE05__ information



## List of Errors

* ERROR_FILE01 - A folder can't be created
    * Context : __module_file_config.rs/path_create_dir()__
    * Cause : see details in logs file to get __create_dir()__ error informations

* ERROR_FILE02 - A folder can't be readed
    * Context : __module_file_config.rs/path_read_check()__
    * Cause : see details in logs file to get __read_dir()__ error informations

* ERROR_FILE03 - Error during removing
    * Context : __module_file_config.rs/delete_if_empty()__
    * Cause : see details in logs file to get __remove_dir()__ error informations

* ERROR_FILE04 - A necessery file doesn't exist
    * Context : __module_file_config.rs/file_exist_check()__
    * Cause : see details in logs file to get __exists()__ error informations

* ERROR_FILE05 - A necessery file can't be readed
    * Context : __module_file_config.rs/file_read_check()__
    * Cause : see details in logs file to get __read_to_string()__ error informations



## List of Warnings

* WARN_LOG01 - Error during default configuration loading
    * Context : __module_log.rs/open_modules()__
    * Info : The file __config_log.yml__ can't be loaded in log4rs configuration
    * Info : A backup logging configuration may be load,logs are in __imbriqua_structure.log__ file
    * Cause : See logs for syntaxe error details, or deserialize error details
