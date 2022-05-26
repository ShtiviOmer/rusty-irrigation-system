use rusty_irrigation_system::config;
use std::env;
use std::process;
use tracing::info;

#[tokio::main]
async fn main() {
    start_tracing();
    info!("Starting...");

    let config = match config::load_from_yaml(get_config_file_path()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = rusty_irrigation_system::run(config).await {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn start_tracing() {
    let format = tracing_subscriber::fmt::format()
        .pretty()
        .with_line_number(true);
    tracing_subscriber::fmt().event_format(format).init();
}

fn get_config_file_path() -> String {
    env::var("IRRIGATION_CONFIG_PATH").unwrap_or_else(|e| {
        tracing::debug!("Failed to read environment variable: {e}");
        "config.json".to_string()
    })
}
