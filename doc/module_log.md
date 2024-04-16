Configure resilient logger : this module provide logger config using two configuration

# How to use

Configure logger with YAML configuration file, after configuring logger with backup configuration (described in next section)

This two-step provide a working detailed logging backup configuration during configuration file error

## Minimal usecase

```
# fn f() {
mod module_log;
use log::{error, warn, info, debug, trace};

fn main() {

    let _handle = module_log::open_module();

    error!("It's an error log");
    warn!("It's a warn log");
    info!("It's an info log");
    debug!("It's a debug log");
    trace!("It's a trace log");

}
# }
# fn main() {}
```

## Customization

Edit config_log.yaml file configuration this log4rs notation

## Example config_log.yaml file configuration

```
appenders:
    stdout:
        kind: console
        encoder:
            pattern: "{M}: {m} {n}"
        filters:
            - kind: threshold
              level: info

    requests:
        kind: file
        path: "imbriqua_structure_loader.log"
        encoder:
            pattern: "{d(%+)(utc)} [{f}:{L}] {h({l})} {M}: {m} {n}"
        filters:
            - kind: threshold
              level: trace

root:
    level: trace
    appenders:
        - stdout
        - requests
```

## Equivalent YAML file of backup configuration

```
appenders:
    stdout:
        kind: console
        encoder:
            pattern: "! BACKUP_LOGGER ! {M}: {m} {n}"
        filters:
            - kind: threshold
              level: trace

    requests:
        kind: file
        path: "imbriqua_structure_loader.log"
        encoder:
            pattern: "{d(%+)(utc)} [{f}:{L}] {h({l})} ! BACKUP_LOGGER ! {M}: {m} {n}"
        filters:
            - kind: threshold
              level: trace

root:
    level: trace
    appenders:
        - stdout
        - requests
```

# Panic and failure

* PANIC_LOG01 - Error during the loading on logs modules
    * Context : __module_log.rs/open_modules()__
    * Info : No logs are provided, so, make panic
    * Cause : The error come from __get_config_by_backup()__ function

* PANIC_LOG02 - Error during the loading on logs modules
    * Context : __module_log.rs/open_modules()__
    * Info : No logs are provided, so, make panic
    * Cause : The error come from __log4rs::init_config()__ function

* WARN_LOG01 - Error during default configuration loading
    * Context : __module_log.rs/open_modules()__
    * Info : The file __config_log.yml__ can't be loaded in log4rs configuration
    * Info : A backup logging configuration may be load,logs are in __imbriqua_structure_loader.log__ file
    * Cause : See logs for syntaxe error details, or deserialize error details
