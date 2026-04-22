use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sensors {
    HeartRate(f64),
    BloodPressure(String),
    BarometricPressure(f64),
    DistanceTraveled(f64),
    Steps(i64),
    WaterDetected(bool),
    Time(f64),
    Temperature(f64),
    Bluetooth(String)
}

impl Sensors {
    pub fn name(&self) -> &'static str {
        match self {
            Sensors::HeartRate(_) => "Heart Rate",
            Sensors::BloodPressure(_) => "Blood Pressure",
            Sensors::BarometricPressure(_) => "Barometric Pressure",
            Sensors::DistanceTraveled(_) => "Distance Traveled",
            Sensors::Steps(_) => "Steps Taken",
            Sensors::Time(_) => "Time",
            Sensors::Temperature(_) => "Temperature Outside",
            Sensors::Bluetooth(_) => "Bluetooth Transmissions",
            Sensors::WaterDetected(_) => "Water in Device",
        }
    }
}
