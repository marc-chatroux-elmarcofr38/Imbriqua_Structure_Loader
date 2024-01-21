# WIKI : How Imbriqua Structure work ?

## Main

## Logs

## List of 

## List of Panics

* PANIC_LOG01 - Error during the loading on logs modules
    * Don't continue with an error during the loading of __module_log.rs/open_modules()__
    * Good luck...

* 

## List of Errors

* ERR_LOG01 - Error during backup configuration loading
    * The error come from __load_configuration_backup__ function or __get_backup_config__ function on __module_log.rs__
    * Logs module can't load __backup configuration__
    * No logs are provided 
    

* ERR_LOG02 - Using backup logging configuration
    * I won't use backup logging configuration during a normal run, so i make a exit

## List of Warnings

* WARN_LOG01 - Error during default configuration loading
    * The file __config_log.yml__ can't be loaded in log configuration
    * A backup logging configuration may be load,logs are in __imbriqua_structure.log__ file
    * See logs for syntaxe error details, or deserialize error details
