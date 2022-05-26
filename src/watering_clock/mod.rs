use std::{fmt, str::FromStr};

use chrono::{self, Timelike};
use tokio::{sync::mpsc, task::JoinHandle, time::Interval};
use tracing::{info_span, instrument::Instrumented, Instrument};

use crate::{config::WateringClockConfig, valve_controller::ValveControllerMessage};

use chrono::NaiveTime;

/// The watering clock is a task that will send valve commands to the valve controller based on time
pub struct WateringClock {
    /// The interval at which to send valve commands in hours, example once a day
    interval: chrono::Duration,
    /// The time at which to start sending valve commands, 09:00
    start_watering_time: NaiveTime,
    /// How long to keep the valve open in minutes
    duration: chrono::Duration,
}

impl TryFrom<WateringClockConfig> for WateringClock {
    type Error = WateringClockError;

    fn try_from(value: WateringClockConfig) -> Result<Self, Self::Error> {
        let start_watering_time = NaiveTime::from_str(&value.start_time)
            .map_err(|e| WateringClockError::FailedLoadConfigError(e.to_string()))?;
        let duration = chrono::Duration::minutes(value.duration);
        let interval = chrono::Duration::hours(value.interval);
        Ok(Self {
            interval,
            start_watering_time,
            duration,
        })
    }
}

impl WateringClock {
    /// tx: The valve controller's tx channel
    pub async fn start(
        &self,
        tx: mpsc::Sender<ValveControllerMessage>,
    ) -> Result<Instrumented<JoinHandle<()>>, WateringClockError> {
        let start_event = get_when_next_time_occurrence(self.start_watering_time)?;
        let end_event = get_when_next_time_occurrence(self.start_watering_time + self.duration)?;

        let mut open_valve_time = get_interval_by_duration(start_event, self.interval)?;

        let mut close_valve_time = get_interval_by_duration(end_event, self.interval)?;

        Ok(tokio::spawn(async move {
            loop {
                open_valve_time.tick().await;
                tracing::info!("Opening Valve");
                if let Err(e) = tx.send(ValveControllerMessage::Open).await {
                    tracing::error!("Error sending open valve command: {}", e);
                    continue;
                }
                close_valve_time.tick().await;
                tracing::info!("Closing Valve");
                if let Err(e) = tx.send(ValveControllerMessage::Close).await {
                    tracing::error!("Error sending close valve command: {}", e);
                    continue;
                }
            }
        })
        .instrument(info_span!("watering_clock")))
    }
}

/// Get the next time that the given time will occur, for example 09:00 if the current time now is 08:00 will return current date at 09:00
/// If current time is 10:00 will return current date + 1 day at 09:00
fn get_when_next_time_occurrence(time: NaiveTime) -> Result<chrono::Duration, WateringClockError> {
    let now = chrono::Local::now();
    let mut time_occurrence = now
        .date()
        .and_hms(time.hour(), time.minute(), 0)
        .signed_duration_since(now);

    if time_occurrence < chrono::Duration::seconds(0) {
        time_occurrence = time_occurrence
            .checked_add(&chrono::Duration::hours(24))
            .ok_or_else(|| {
                WateringClockError::StartClockError("overflow of clock occurred".to_string())
            })?;
    }
    Ok(time_occurrence)
}

/// Get the interval at which to send valve commands, example once a day at 09:00
fn get_interval_by_duration(
    next_event: chrono::Duration,
    interval: chrono::Duration,
) -> Result<Interval, WateringClockError> {
    Ok(tokio::time::interval_at(
        tokio::time::Instant::now()
            + next_event
                .to_std()
                .map_err(|e| WateringClockError::StartClockError(e.to_string()))?,
        interval
            .to_std()
            .map_err(|e| WateringClockError::StartClockError(e.to_string()))?,
    ))
}

#[derive(Debug)]
pub enum WateringClockError {
    StartClockError(String),
    FailedLoadConfigError(String),
}

impl std::error::Error for WateringClockError {}

impl fmt::Display for WateringClockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WateringClockError::StartClockError(e) => write!(f, "Error starting clock: {}", e),
            WateringClockError::FailedLoadConfigError(e) => {
                write!(f, "Error loading config: {}", e)
            }
        }
    }
}
