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
#![doc = include_str!("../doc/module_log.md")]

// Package section
use crate::module_file_manager::{FileManager, Path};

// Dependencies section
pub use log::{error, warn, info, trace, debug};


/// Configure logger with __config_file__, after configure it with a backup configuation
///
/// Initialise the logger with __config_file__
/// Provide minimal logger during initialise on __config_file__, and after if a error has meet
pub fn open_logger(config_file : &str) -> log4rs::Handle {

    // Itinialisation of global logger with backup configuration
    let config : log4rs::config::Config = match get_config_by_backup() {
        Ok(result) => {result},
        Err(error) => {
            error!("PANIC_LOG01 - Error during the loading on logs modules - {}", error);
            panic!("PANIC_LOG01 - Error during the loading on logs modules")},

    };
    let handle : log4rs::Handle = match log4rs::init_config(config) {
        Ok(result) => {result},
        Err(error) => {
            error!("PANIC_LOG02 - Error during the loading on logs modules - {}", error);
            panic!("PANIC_LOG02 - Error during the loading on logs modules")
        },
    };

    // Itinialisation of global logger with "config_file" configuration
    let config : log4rs::config::Config = match get_config_by_file(config_file) {
        Ok(result) => {
            result
        },
        Err(error) => {
            warn!("WARN_LOG01 - Error during default configuration loading \"{}\": {}", config_file, error);
            info!("Logger init success : use of \"!!! BACKUP CONFIGURATION !!!\"");
            return handle
        },
    };
    handle.set_config(config);

    // Return
    info!("Log handle loaded");
    handle
}

#[doc(hidden)]
/// Define a backup config (hard-writted)
fn get_config_by_backup() -> anyhow::Result<log4rs::config::Config> {

    // Setup of console logging output
    let stdout : log4rs::append::console::ConsoleAppender = log4rs::append::console::ConsoleAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("! BACKUP_LOGGER ! {M}: {m} {n}")))
        .build();

    // Setup of file logging tools
    let requests : log4rs::append::file::FileAppender = log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{d(%+)(utc)} [{f}:{L}] {h({l})} ! BACKUP_LOGGER ! {M}: {m} {n}")))
        .build("imbriqua_structure_loader.log")?;

    // Setup of global logger
    let config : log4rs::config::Config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("stdout", Box::new(stdout)))
        .appender(log4rs::config::Appender::builder().build("requests", Box::new(requests)))
        .logger(log4rs::config::Logger::builder().appender("stdout").build("app::stdout", log::LevelFilter::Trace))
        .logger(log4rs::config::Logger::builder().appender("requests").build("app::requests", log::LevelFilter::Trace))
        .build(log4rs::config::Root::builder().appender("stdout").appender("requests").build(log::LevelFilter::Trace))?;

    Ok(config)
}

#[doc(hidden)]
/// Define a config by loading "config_file"
fn get_config_by_file(config_file : &str) -> anyhow::Result<log4rs::config::Config> {

    let default_config_string : String = Path::new(config_file).get_file_content();
    let default_config_str = default_config_string.as_str();

    // Deserialize
    let config : log4rs::config::RawConfig = serde_yaml::from_str(default_config_str)?;
    let (appenders, errors) = config.appenders_lossy(&log4rs::config::Deserializers::default());

    // Error test
    if !errors.is_empty() {
        return Err(anyhow::Error::new(errors));
    }

    // Initialise config object
    let config : log4rs::config::Config = log4rs::config::Config::builder()
        .appenders(appenders)
        .loggers(config.loggers())
        .build(config.root())?;

    Ok(config)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialize_log_for_test() {
        INIT.call_once(|| {
            open_logger("tests/config_log_for_test.yml");
        });
    }

    #[test]
    fn module_log_01_check_configuration_by_backup() {
        // Checking execution
        let result = get_config_by_backup();
        // Checking Result
        assert!(result.is_ok());
    }

    #[test]
    fn module_log_02_check_configuration_by_file() {
        // Checking execution
        let result = get_config_by_file("tests/module_log/module_log_02_check_configuration_by_file/config_log.yml");
        // Checking Result
        assert!(result.is_ok());
    }

    #[ignore]
    #[test]
    fn module_log_03_check_open_logger() {
        initialize_log_for_test();
    }
}
