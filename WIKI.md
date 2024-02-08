# WIKI : How Imbriqua Structure work ?

## Main

## Logs

## List of 

## List of Panics

* PANIC_LOG01 - Error during the loading on logs modules
    * Context : __module_log.rs/open_modules()__
    * Info : No logs are provided, so, make panic
    * Cause : The error come from __load_configuration_backup()__ function or __get_backup_config()__
        

* 

## List of Errors

* 

## List of Warnings

* WARN_LOG01 - Error during default configuration loading
    * Context : __module_log.rs/open_modules()__
    * Info : The file __config_log.yml__ can't be loaded in log4rs configuration
    * Info : A backup logging configuration may be load,logs are in __imbriqua_structure.log__ file
    * Cause : See logs for syntaxe error details, or deserialize error details
