use std::{cell::RefCell, collections::HashMap};

use serde::{Deserialize, Serialize};
use worker::{SetAlarmOptions, State};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ScheduledAlarm {
    DatabaseSync,
    // Add more alarm types as needed
}

impl ScheduledAlarm {
    pub fn initial_alarms() -> Vec<ScheduledAlarm> {
        vec![
            ScheduledAlarm::DatabaseSync,
            // Add more initial alarms as needed
        ]
    }
}

pub fn default_options() -> SetAlarmOptions {
    SetAlarmOptions {
        allow_concurrency: Some(true),
        allow_unconfirmed: Some(true),
    }
}

pub async fn update_system_alarm(
    state: &State,
    alarms: &RefCell<HashMap<i64, Vec<ScheduledAlarm>>>,
) -> worker::Result<()> {
    let keys = alarms.borrow().keys().cloned().collect::<Vec<i64>>();
    let storage = state.storage();
    let alarm_options = SetAlarmOptions {
        allow_concurrency: Some(true),
        allow_unconfirmed: Some(true),
    };
    if let Some(&earliest_time) = keys.iter().min() {
        if let Err(e) = storage
            .set_alarm_with_options(earliest_time, alarm_options)
            .await
        {
            let err_msg = format!("Failed to set system alarm: {}", e);
            return Err(worker::Error::Internal((&err_msg).into()));
        }
    } else if let Err(e) = storage.delete_alarm().await {
        let err_msg = format!("Failed to delete system alarm: {}", e);
        return Err(worker::Error::Internal((&err_msg).into()));
    }
    Ok(())
}
