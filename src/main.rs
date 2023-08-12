use rusty_irrigation_system::config;
use std::env;
use std::process;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

#[tokio::main]
async fn main() {
    let _guard = start_tracing();
    info!("Starting...");

    let config = match config::load_from_yaml(get_config_file_path()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {:?}", e);
            process::exit(1);
        }
    };
    if let Err(e) = rusty_irrigation_system::run(config).await {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn start_tracing() -> Option<WorkerGuard> {
    let log_to_file = env::var("LOG_TO_FILE")
        .unwrap_or_else(|_| "true".to_owned())
        .parse::<bool>()
        .expect("LOG_TO_FILE Param must be true or false");
    let fmt = tracing_subscriber::fmt();
    let format = tracing_subscriber::fmt::format()
        .pretty()
        .with_line_number(true);
    let fmt = fmt.event_format(format);
    let guard = if log_to_file == true {
        let log_path =
            env::var("LOG_PATH").unwrap_or_else(|_| "/var/log/rusty_irrigation_system/".to_owned());
        let file_appender = tracing_appender::rolling::daily(log_path, "rusty_irrigation.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        fmt.with_writer(non_blocking).init();
        Some(guard)
    } else {
        fmt.init();
        None
    };
    guard
}

fn get_config_file_path() -> String {
    env::var("IRRIGATION_CONFIG_PATH").unwrap_or_else(|e| {
        tracing::debug!("Failed to read environment variable: {e}");
        "/etc/rusty_irrigation_system/config.yaml".to_string()
    })
}
