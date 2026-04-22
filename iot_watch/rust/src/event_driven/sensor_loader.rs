use std::collections::HashMap;
use std::fs;

use crate::event_driven::sensors::Sensors;

pub struct SensorLoader {
    data: HashMap<String, Vec<HashMap<String, serde_json::Value>>>,
    index: usize,
}

impl SensorLoader {
    pub fn new(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read JSON");
        let data: HashMap<String, Vec<HashMap<String, serde_json::Value>>> =
            serde_json::from_str(&content).expect("Invalid JSON");

        Self { data, index: 1 }
    }

    pub fn next_snapshot(&mut self) -> Vec<Sensors> {
        let key = self.index.to_string();

        let snapshot = self.data.get(&key).cloned().unwrap_or_default();

        self.index += 1;

        snapshot
            .into_iter()
            .filter_map(Self::map_to_sensor)
            .collect()
    }

    fn map_to_sensor(entry: HashMap<String, serde_json::Value>) -> Option<Sensors> {
        let (key, value) = entry.into_iter().next()?;

        match key.as_str() {
            "Heart Rate" => Some(Sensors::HeartRate(value.as_f64()?)),
            "Blood Pressure" => Some(Sensors::BloodPressure(value.as_str()?.to_string())),
            "Barometric Pressure" => Some(Sensors::BarometricPressure(value.as_f64()?)),
            "Distance Traveled" => Some(Sensors::DistanceTraveled(value.as_f64()?)),
            "Steps Taken" => Some(Sensors::Steps(value.as_i64()?)),
            "Time" => Some(Sensors::Time(value.as_f64()?)),
            "Temperature Outside" => Some(Sensors::Temperature(value.as_f64()?)),
            "Bluetooth Transmissions" => Some(Sensors::Bluetooth(value.as_str()?.to_string())),
            "Water in Device" => Some(Sensors::WaterDetected(value.as_bool()?)),
            _ => None,
        }
    }
}