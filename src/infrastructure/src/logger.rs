use simple_logger::SimpleLogger;

pub fn setup_logger() {
    SimpleLogger::new().env().init().expect("Failed to initialize logger")
}
