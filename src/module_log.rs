use anyhow::Result;
use log::{info, warn, LevelFilter};
use log4rs::{init_raw_config, init_config};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
use serde_yaml::from_str;

pub fn open_module() {

    // Exit if error
    if sub_open_module().is_err() {
        panic!("Error during the loading on logs modules")
    }
}


fn sub_open_module() -> Result<()> {
    /*
        main fucntion for configuration of gloabal logger
        Try to load the configuration file, else try to load a backup configuration
    */

    match load_configuration_file() {
        Ok(result) => {
            info!("Logger init success : use of \"{}\"", "config_log.yml");
            Ok(result)
        },
        Err(error) => {
            match load_configuration_backup() {
                Ok(result) => {
                    info!("Logger init success : use of \"{}\"", "!!! BACKUP CONFIGURATION !!!");
                    warn!("Error during default configuration loading : {}", error);
                    Ok(result)
                },
                Err(error) => {
                    panic!("Error during backup configuration loading {}", error);
                },
            }
        },
    }
}



fn load_configuration_file() -> Result<()> {
    /*
        Try to load a default configuration for global logger.
        It use the following file : "config_log.yml"
    */

    // Loading of file
    let default_config_str = include_str!("config_log.yml");

    // Setup of global logger
    let config = from_str(default_config_str)?;

    // Itinialisation of global logger
    init_raw_config(config)?;

    Ok(())
}


fn load_configuration_backup() -> Result<()> {
    /*
        Try to load a backup configuration for global logger.
        Use a "hard writted" configuration
    */

    // Setup of console logging output
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{M}: {m} {n}")))
        .build();

    // Setup of file logging tools
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%+)(utc)} [{f}:{L}] {h({l})} ! BACKUP_LOGGER ! {M}: {m} {n}")))
        .build("imbriqua_engine.log")?;

    // Setup of global logger
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().appender("stdout").build("app::stdout", LevelFilter::Debug))
        .logger(Logger::builder().appender("requests").build("app::requests", LevelFilter::Debug))
        .build(Root::builder().appender("stdout").appender("requests").build(LevelFilter::Debug))?;

    // Itinialisation of global logger
    init_config(config)?;

    Ok(())
}
