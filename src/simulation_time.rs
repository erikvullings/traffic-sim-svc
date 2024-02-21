// use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};
use tokio::sync::mpsc;

use crate::consumer::KafkaConsumer;
/// Initialization – preparing for the actual start of the simulation time; Started – the simulation time is started; Paused – the simulation time is paused; Stopped – the simulation time is stopped; Reset – the simulation time is reset
#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum TimeState {
    Initialization,
    Started,
    Paused,
    Stopped,
    Reset,
}

/// The time management message can be used for informing connected applications on time progression and changes. *Copyright (C) 2019-2020 XVR Simulation B.V., Delft, The Netherlands, Martijn Hendriks <hendriks @ xvrsim.com>. This file is licensed under the MIT license : https: //github.com/DRIVER-EU/avro-schemas/blob/master/LICENSE*
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct TimeManagement {
    pub state: TimeState,
    #[serde(default = "default_timemanagement_tags")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(default = "default_timemanagement_timestamp")]
    pub timestamp: Option<i64>,
    #[serde(rename = "simulationTime")]
    #[serde(default = "default_timemanagement_simulation_time")]
    pub simulation_time: Option<i64>,
    #[serde(rename = "simulationSpeed")]
    #[serde(default = "default_timemanagement_simulation_speed")]
    pub simulation_speed: Option<f32>,
}

#[inline(always)]
fn default_timemanagement_tags() -> Option<::std::collections::HashMap<String, String>> {
    None
}

#[inline(always)]
fn default_timemanagement_timestamp() -> Option<i64> {
    None
}

#[inline(always)]
fn default_timemanagement_simulation_time() -> Option<i64> {
    None
}

#[inline(always)]
fn default_timemanagement_simulation_speed() -> Option<f32> {
    None
}

pub struct SimulationTimer {
    updated: Arc<Mutex<DateTime<Utc>>>,
    time: Arc<Mutex<DateTime<Utc>>>,
    state: Arc<Mutex<TimeState>>,
    speed: Arc<Mutex<f32>>,
    handle: tokio::task::JoinHandle<()>,
}

impl Drop for SimulationTimer {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

impl SimulationTimer {
    pub async fn new(
        kafka_host: &String,
        schema_registry_url: &String,
        group_id: &String,
    ) -> SimulationTimer {
        let consumer = KafkaConsumer::new(
            kafka_host.to_string(),
            schema_registry_url.to_string(),
            group_id.to_string(),
            "simulation_time_mgmt".to_string(),
        );
        let (sender, mut receiver) = mpsc::unbounded_channel::<TimeManagement>();
        let handle = tokio::spawn(async move {
            consumer.consume(sender.clone()).await;
        });

        let now = SystemTime::now();
        let updated = Arc::new(Mutex::new(DateTime::<Utc>::from(now)));
        let time = Arc::new(Mutex::new(DateTime::<Utc>::from(now)));
        let state = Arc::new(Mutex::new(TimeState::Initialization));
        let speed = Arc::new(Mutex::new(0_f32));

        let sim_timer = SimulationTimer {
            updated: updated.clone(),
            time: time.clone(),
            state: state.clone(),
            speed: speed.clone(),
            handle,
        };

        let updated_clone = updated.clone();
        let time_clone = time.clone();
        let state_clone = state.clone();
        let speed_clone = speed.clone();

        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                let now = SystemTime::now();
                let mut updated = updated_clone.lock().unwrap();
                *updated = DateTime::<Utc>::from(now);
                let mut state = state_clone.lock().unwrap();
                *state = message.state;
                match message.simulation_time {
                    Some(sim_time) => {
                        // println!("Sim time: {}", sim_time);
                        let mut time = time_clone.lock().unwrap();
                        *time = DateTime::from_timestamp_millis(sim_time).unwrap();
                    }
                    _ => (),
                }
                let mut speed = speed_clone.lock().unwrap();
                match message.simulation_speed {
                    Some(sim_speed) => {
                        // println!("Sim speed: {}", sim_speed);
                        *speed = sim_speed;
                    }
                    _ => {
                        *speed = 1.0;
                    }
                }
            }
        });

        sim_timer
    }

    pub fn get_time(&self) -> DateTime<Utc> {
        let now = DateTime::<Utc>::from(SystemTime::now()).timestamp_millis();
        let updated = self.updated.lock().unwrap().timestamp_millis();
        let time_passed_millis = now - updated;
        let speed = self.speed.lock().unwrap();
        let sim_time_passed_millis = ((time_passed_millis as f32) * *speed).round() as i64;
        let time = self.time.lock().unwrap();
        DateTime::from_timestamp_millis(time.timestamp_millis() + sim_time_passed_millis).unwrap()
    }

    pub fn get_state(&self) -> TimeState {
        let state = self.state.lock().unwrap();
        (*state).clone()
    }

    pub fn get_speed(&self) -> f32 {
        let speed = self.speed.lock().unwrap();
        (*speed).clone()
    }
}

// pub fn get_simulation_time(&mut sim_time: Date) {
//     let now: DateTime = DateTime::now();
// }
