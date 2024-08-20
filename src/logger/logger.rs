use env_logger::Builder;
use std::io::Write;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

/// Create log file and set up the logger.
pub fn init_logging() {
    // Create and open log file.
    let log_file =
        std::fs::File::create(format!("{}.log", PKG_NAME)).expect("Error creating log file");
    // Only one thread allowed.
    let log_file = std::sync::Mutex::new(log_file);

    // Set up the logger
    Builder::new()
        .format(move |_buf, record| {
            let mut log_file = log_file.lock().unwrap();
            writeln!(
                log_file,
                "{} [{}] - {}:{} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Info)
        .init();
}
