use rusty_irrigation_system::{Config, ValveType};
use std::process;
use tokio;
use tracing::info;

#[tokio::main]
async fn main() {
    start_tracing();
    info!("Starting...");
    #[cfg(test)]
    info!("Valve is mock");
    #[cfg(test)]
    let valve_type = ValveType::Mock;
    #[cfg(not(test))]
    info!("Valve is RaspberryPie");
    #[cfg(not(test))]
    let valve_type = ValveType::RaspberryPie;

    let config = Config::new(valve_type);

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
