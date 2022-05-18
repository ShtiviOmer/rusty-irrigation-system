use rusty_irrigation_system::{Config, ValveType};
use std::process;
use tokio;

#[tokio::main]
async fn main() {
    #[cfg(test)]
    let valve_type = ValveType::Mock;
    #[cfg(not(test))]
    let valve_type = ValveType::RaspberryPie;
    let config = Config::new(valve_type);

    if let Err(e) = rusty_irrigation_system::run(config).await {
        eprintln!("{}", e);
        process::exit(1);
    }
}
