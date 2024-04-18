Configure resilient logger : this module provide logger config using two configuration

# How to use

Configure logger with YAML configuration file, after configuring logger with backup configuration (described in next section)

This two-step provide a working detailed logging backup configuration when configuration file provide errors (minimal available logger before file configuration logger)

## Minimal usecase (with __config_log.yml__ file)

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

Use __pub mod module_log__ in __main.rs__.

This practice allowing to import logs macro (__error!__, __warn!__, __info!__, __debug!__, __trace!__) by using __use crate::module_log::*;__


```rust
// main.rs
pub mod module_log

fn main () {
    // ...
}
```

```rust
// foo.rs
use crate::module_log::*;

fn bae () {
    // ...

    info!("Foo!, Bar!");

    // ...
}

```

## Customization

Edit config_log.yml file configuration with log4rs notation (or use a other file)

## Example __config_log.yml__ file configuration

```yaml
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

## Equivalent YAML file of backup configuration (module hard-writted in module code)

```yaml
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
    * Context : __module_log.rs/open_loggers()__
    * Info : No logs are provided, so, make panic
    * Cause :
        * The error come from __get_config_by_backup()__ function
        * Error come from coding mistake, or libraries changes

* PANIC_LOG02 - Error during the loading on logs modules
    * Context : __module_log.rs/open_loggers()__
    * Info : No logs are provided, so, make panic
    * Cause :
        * The error come from __log4rs::init_config()__ function
        * Logger initialise a second time, or __Config__ mistake

* WARN_LOG01 - Error during default configuration loading
    * Context : __module_log.rs/open_loggers()__
    * Info : The file __config_log.yml__ can't be loaded in log4rs configuration
    * Info : A backup logging configuration may be load, logs are in __imbriqua_structure_loader.log__ file
    * Cause : See logs for syntaxe error details, or deserialize error details
