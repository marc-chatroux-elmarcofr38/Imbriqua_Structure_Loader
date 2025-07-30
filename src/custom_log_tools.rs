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

#![warn(dead_code)]
#![warn(missing_docs)]
#![allow(unused)]
#![doc = include_str!("../doc/custom_log_tools.md")]

// Package section
use crate::custom_file_tools::*;

// Dependencies section
use anyhow::{Error, Result};
use log::LevelFilter;
pub use log::{debug, error, info, trace, warn};
use log4rs::append::{console::ConsoleAppender, file::FileAppender};
use log4rs::config::{Appender, Config, Deserializers, Logger, RawConfig, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::init_config;
use log4rs::Handle;
use std::fmt::Debug;

/// Configure logger with __config_file__, after configure it with a backup configuation
///
/// Initialise the logger with __config_file__
/// Provide minimal logger during initialise on __config_file__, and after if a error has meet
pub fn open_logger(config_file: &str) -> Result<Handle, anyhow::Error> {
    // Itinialisation of global logger with backup configuration
    let config: Config = match get_config_by_backup() {
        Ok(result) => result,
        Err(error) => {
            error!(
                "PANIC_LOG01 - Error during the loading on logs modules - {}",
                error
            );
            return Err(anyhow::format_err!(
                "PANIC_LOG01 - Error during the loading on logs modules - {}",
                error
            ));
        }
    };
    let handle: Handle = match init_config(config) {
        Ok(result) => result,
        Err(error) => {
            error!(
                "PANIC_LOG02 - Error during the loading on logs modules - {}",
                error
            );
            return Err(anyhow::format_err!(
                "PANIC_LOG02 - Error during the loading on logs modules - {}",
                error
            ));
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
            return Ok(handle);
        }
    };
    handle.set_config(config);

    // Return
    info!("Log handle loaded");
    Ok(handle)
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
    let default_config_string: String = Path::new(config_file).get_file_content().unwrap();
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

/// Make a 'trace' level log of an object, if the input was an error
///
/// Input :
///     - object_result : Result<T1, anyhow::Error>,
///         - value to evaluate
///     - object : T2
///         - value to log ('trace')
///
/// Output :
///     - return object_result
pub fn catch_error_and_log<T1, T2: Debug>(
    object_result: Result<T1, anyhow::Error>,
    object: &T2,
) -> Result<T1, anyhow::Error> {
    if object_result.is_err() {
        trace!("catch_error_and_log : {:#?}", object);
        object_result
    } else {
        object_result
    }
}

/// Test on the log module
#[cfg(test)]
pub mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    /// Provide logs as set in "tests/custom_log_tools/config_log_for_test.yml"
    /// Provide logs for test in others module in "tests/tests.log"
    /// by passing "use crate::custom_log_tools::tests::initialize_log_for_test;"
    ///
    /// ```rust
    /// #[cfg(test)]
    /// mod tests {
    ///     use super::*;
    ///     use crate::custom_log_tools::tests::initialize_log_for_test;
    /// ```
    pub fn initialize_log_for_test() {
        INIT.call_once(|| {
            open_logger("tests/custom_log_tools/config_log_for_test.yml");
            info!("\n\n\n\n\nNEW SESSION OF TEST\n\n");
        });
    }

    #[test]
    fn custom_log_tools_01_check_configuration_by_backup() {
        // Checking execution
        let result = get_config_by_backup();
        // Checking Result
        assert!(result.is_ok());
    }

    #[test]
    fn custom_log_tools_02_check_configuration_by_file() {
        // Checking execution
        let result = get_config_by_file(
            "tests/custom_log_tools/custom_log_tools_02_check_configuration_by_file/config_log.yml",
        );
        // Checking Result
        assert!(result.is_ok());
    }

    #[test]
    fn custom_log_tools_03_check_open_logger() {
        initialize_log_for_test();
        // just check if don't panic
        trace!("LOG TEST : TRACE");
        debug!("LOG TEST : DEBUG");
        info!("LOG TEST : INFO");
        warn!("LOG TEST : WARN");
        error!("LOG TEST : ERROR");
    }

    #[test]
    fn custom_log_tools_04_catch_error_and_log() {
        initialize_log_for_test();
        // make an input
        let i_1: std::result::Result<String, anyhow::Error> = Ok(String::new());
        let i_1_clone: std::result::Result<String, anyhow::Error> = Ok(String::new());
        let i_2: std::result::Result<String, anyhow::Error> = Err(anyhow::format_err!("an error"));
        let i_2_clone: std::result::Result<String, anyhow::Error> =
            Err(anyhow::format_err!("an error"));
        // just check if don't panic
        let r_1 = catch_error_and_log(i_1, &String::new());
        let r_2 = catch_error_and_log(i_2, &String::new());
        assert_eq!(i_1_clone.is_err(), r_1.is_err());
        assert_eq!(i_1_clone.unwrap(), r_1.unwrap());
        assert_eq!(i_2_clone.is_err(), r_2.is_err());
    }
}
