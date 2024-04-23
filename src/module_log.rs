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
use anyhow::{Error, Result};
use log::LevelFilter;
pub use log::{debug, error, info, trace, warn};
use log4rs::append::{console::ConsoleAppender, file::FileAppender};
use log4rs::config::{Appender, Config, Deserializers, Logger, RawConfig, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::init_config;
use log4rs::Handle;

/// Configure logger with __config_file__, after configure it with a backup configuation
///
/// Initialise the logger with __config_file__
/// Provide minimal logger during initialise on __config_file__, and after if a error has meet
pub fn open_logger(config_file: &str) -> Handle {
    // Itinialisation of global logger with backup configuration
    let config: Config = match get_config_by_backup() {
        Ok(result) => result,
        Err(error) => {
            error!(
                "PANIC_LOG01 - Error during the loading on logs modules - {}",
                error
            );
            panic!(
                "PANIC_LOG01 - Error during the loading on logs modules - {}",
                error
            );
        }
    };
    let handle: Handle = match init_config(config) {
        Ok(result) => result,
        Err(error) => {
            error!(
                "PANIC_LOG02 - Error during the loading on logs modules - {}",
                error
            );
            panic!(
                "PANIC_LOG02 - Error during the loading on logs modules - {}",
                error
            );
        }
    };

    // Itinialisation of global logger with "config_file" configuration
    let config: Config = match get_config_by_file(config_file) {
        Ok(result) => result,
        Err(error) => {
            warn!(
                "WARN_LOG01 - Error during default configuration loading \"{}\": {}",
                config_file, error
            );
            info!("Logger init success : use of \"!!! BACKUP CONFIGURATION !!!\"");
            return handle;
        }
    };
    handle.set_config(config);

    // Return
    info!("Log handle loaded");
    handle
}

#[doc(hidden)]
/// Define a backup config (hard-writted)
fn get_config_by_backup() -> Result<Config> {
    // Setup of console logging output
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "! BACKUP_LOGGER ! {M}: {m} {n}",
        )))
        .build();

    // Setup of file logging tools
    let requests: FileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%+)(utc)} [{f}:{L}] {h({l})} ! BACKUP_LOGGER ! {M}: {m} {n}",
        )))
        .build("imbriqua_structure_loader.log")?;

    // Setup of global logger
    let config: Config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(
            Logger::builder()
                .appender("stdout")
                .build("app::stdout", LevelFilter::Trace),
        )
        .logger(
            Logger::builder()
                .appender("requests")
                .build("app::requests", LevelFilter::Trace),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("requests")
                .build(LevelFilter::Trace),
        )?;

    Ok(config)
}

#[doc(hidden)]
/// Define a config by loading "config_file"
fn get_config_by_file(config_file: &str) -> Result<Config> {
    let default_config_string: String = Path::new(config_file).get_file_content();
    let default_config_str = default_config_string.as_str();

    // Deserialize
    let config: RawConfig = serde_yaml::from_str(default_config_str)?;
    let (appenders, errors) = config.appenders_lossy(&Deserializers::default());

    // Error test
    if !errors.is_empty() {
        return Err(Error::new(errors));
    }

    // Initialise config object
    let config: Config = Config::builder()
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
            open_logger("tests/module_log/config_log_for_test.yml");
            info!("\n\n\n\n\nNEW SESSION OF TEST\n\n");
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
        let result = get_config_by_file(
            "tests/module_log/module_log_02_check_configuration_by_file/config_log.yml",
        );
        // Checking Result
        assert!(result.is_ok());
    }

    #[test]
    fn module_log_03_check_open_logger() {
        initialize_log_for_test();
        trace!("LOG TEST : TRACE");
        debug!("LOG TEST : DEBUG");
        info!("LOG TEST : INFO");
        warn!("LOG TEST : WARN");
        error!("LOG TEST : ERROR");
    }
}
