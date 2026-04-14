#[derive(Debug, Clone)]
pub struct SensorFrame {
    pub heart_rate: f64,
    pub blood_pressure: String,
    pub barometric_pressure: f64,
    pub distance_traveled: f64,
    pub steps_taken: i64,
    pub time: f64,
    pub temperature_outside: f64,
    pub bluetooth_transmissions: String,
    pub water_in_device: bool,
}

impl Default for SensorFrame {
    fn default() -> Self {
        Self {
            heart_rate: 0.0,
            blood_pressure: String::new(),
            barometric_pressure: 0.0,
            distance_traveled: 0.0,
            steps_taken: 0,
            time: 0.0,
            temperature_outside: 0.0,
            bluetooth_transmissions: "NONE".to_string(),
            water_in_device: false,
        }
    }
}