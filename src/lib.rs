// TODO: remove this later in development when we have more code
#![allow(dead_code)]
mod saveloading;

/// A macro that sets up logging for tests.
#[cfg(test)]
#[macro_export]
macro_rules! setup_logging {
    ($test_name: expr) => {
        simplelog::CombinedLogger::init(vec![
            simplelog::TermLogger::new(
                simplelog::LevelFilter::Debug,
                simplelog::Config::default(),
                simplelog::TerminalMode::Mixed,
                simplelog::ColorChoice::Auto,
            ),
            simplelog::WriteLogger::new(
                simplelog::LevelFilter::Trace,
                simplelog::Config::default(),
                std::fs::File::create(concat!($test_name, ".log")).unwrap(),
            ),
        ])
        .unwrap();
    };
}
