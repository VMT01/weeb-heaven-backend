use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config,
};

pub fn setup_logger() {
    let encoder = "[{d(%d-%m-%YT%H:%M:%S%.3f)} {h({l})}]\n[{f}:{L}]\n{m}\n\n";

    let stdout = Appender::builder()
        .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
        .build(
            "stdout",
            Box::new(
                ConsoleAppender::builder()
                    .encoder(Box::new(PatternEncoder::new(encoder)))
                    .build(),
            ),
        );

    let logfile = Appender::builder().build(
        "logfile",
        Box::new(
            FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(encoder)))
                .append(false)
                .build(".log")
                .unwrap_or_else(|err| panic!("Error while building log file: {}", err)),
        ),
    );

    let config = Config::builder()
        .appenders([stdout, logfile])
        .build(
            Root::builder()
                .appenders(["stdout", "logfile"])
                .build(LevelFilter::Debug),
        )
        .unwrap_or_else(|err| panic!("Error while building log4rs config: {}", err));

    log4rs::init_config(config)
        .unwrap_or_else(|err| panic!("Error while init log4rs config: {}", err));
}
