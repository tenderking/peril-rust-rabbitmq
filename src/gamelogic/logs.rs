use crate::routing::GameLog;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn init_logger() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("game.log")
        .expect("error creating log file");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .expect("error creating log file");

    log4rs::init_config(config).expect("error initializing log4rs");
}

pub fn write_log(game_log: GameLog) -> Result<(), Box<dyn std::error::Error>> {
    log::info!(
        "{} {}: {}",
        game_log.current_time,
        game_log.username,
        game_log.message.as_str()
    );

    Ok(())
}
