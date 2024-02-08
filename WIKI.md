# WIKI : How Imbriqua Structure work ?

## Main

## Logs

## List of 

## List of Panics

* PANIC_LOG01 - Error during the loading on logs modules
    * Context : __module_log.rs/open_modules()__
    * Info : No logs are provided, so, make panic
    * Cause : The error come from __load_configuration_backup()__ function or __get_backup_config()__
        
* PANIC_FILE01 - Input main folder isn't readable
    * Panic action caused by __ERROR_FILE01__
    * See ERROR_FILE01 information
        
* PANIC_FILE02 - Output main folder isn't readable
    * Panic action caused by __ERROR_FILE02__
    * See ERROR_FILE01 information
        
* PANIC_FILE03 - Error during creation and controle of output subfolder
    * Panic action caused by __ERROR_FILE03__ or __ERROR_FILE04__
    * See ERROR_FILE01 information

## List of Errors

* ERROR_FILE01 - Input main folder isn't readable
    * Context : __module_file_config.rs/get_input_folder()__
    * Cause : see details for get __read_dir()__ error informations

* ERROR_FILE02 - Output main folder isn't readable
    * Context : __module_file_config.rs/get_output_folder()__
    * Cause : see details for get __read_dir()__ error informations

* ERROR_FILE03 - Output subfolder uncreatable 
    * Context : __module_file_config.rs/get_output_folder()__
    * Cause : see details for get __create_dir()__ error informations

* ERROR_FILE04 - Output subfolder isn't readable
    * Context : __module_file_config.rs/get_output_folder()__
    * Cause : see details for get __read_dir()__ error informations

## List of Warnings

* WARN_LOG01 - Error during default configuration loading
    * Context : __module_log.rs/open_modules()__
    * Info : The file __config_log.yml__ can't be loaded in log4rs configuration
    * Info : A backup logging configuration may be load,logs are in __imbriqua_structure.log__ file
    * Cause : See logs for syntaxe error details, or deserialize error details
