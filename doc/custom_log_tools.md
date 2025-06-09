Configure resilient logger : this module provide logger config using two configuration

# How to use

Configure logger with YAML configuration file, after configuring logger with backup configuration (described in next section)

This two-step provide a working detailed logging backup configuration when configuration file provide errors (minimal available logger before file configuration logger)

## Minimal usecase (with __config_log.yml__ file)

```rust
mod custom_log_tools;

fn main() {

    let _handle = custom_log_tools::open_logger("config_log.yml");

    error!("It's an working error log");
    warn!("It's a working warn log");
    info!("It's an working info log");
    debug!("It's a working debug log");
    trace!("It's a working trace log");

}
```

## Good practice

Use [`crate::custom_log_tools`] in each rust file, replacing `log`, `log4rs`, and `env_logger`, with the following command :

* __main.rs__ : `pub mod custom_log_tools;`
* random file : `use crate::custom_log_tools::*;`

This practice allowing you to import logs macro (`error!()`, `warn!()`, `info!()`, `debug!()`, `trace!()`) in each rust file

```rust
// main.rs
pub mod custom_log_tools;

fn main () {
    // ...
}
```

```rust
// foo.rs
use crate::custom_log_tools::*;

fn bar () {
    // ...

    info!("Foo!, Bar!");

    // ...
}

```

## Customization

Edit __config_log.yml__ file configuration with log4rs notation (or use a other file)

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
    * Context : [`open_logger`]
    * Info : No logs are provided, so, make panic
    * Cause :
        * The error come from `get_config_by_backup` function
        * Error come from coding mistake, or libraries changes

* PANIC_LOG02 - Error during the loading on logs modules
    * Context : [`open_logger`]
    * Info : No logs are provided, so, make panic
    * Cause :
        * The error come from `log4rs::init_config` function
        * Logger initialise a second time, or __Config__ mistake

* WARN_LOG01 - Error during default configuration loading
    * Context : [`open_logger`]
    * Info : The file __config_log.yml__ can't be loaded in log4rs configuration
    * Info : A backup logging configuration may be load, logs are in __imbriqua_structure_loader.log__ file
    * Cause : See logs for syntaxe error details, or deserialize error details

# Information for development

During writing of tests function, use the `tests::initialize_log_for_test` function. It's a 'one-call' function providing after this first call.

Logs are provided in __tests/tests.log__ file, using the configuration file __tests/config_log_for_test.yml__

```rust
#[cfg(test)]
mod tests {
    use crate::custom_log_tools::tests::initialize_log_for_test;
    // Other import...

    #[test]
    fn my_test () {
        // Logs setting
        initialize_log_for_test();
        // Test, performing logs output (or not of course)
        // ...
    }
}
```