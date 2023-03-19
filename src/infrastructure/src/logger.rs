use simplelog::{format_description, ColorChoice, Config, ConfigBuilder, TermLogger, TerminalMode};

pub fn setup_logger() {
    let config = ConfigBuilder::new().set_time_format_custom(format_description!("")).build();
    TermLogger::init(log::LevelFilter::Info, config, TerminalMode::Mixed, ColorChoice::Auto).expect("Failed to initialize terminal logger");
}
