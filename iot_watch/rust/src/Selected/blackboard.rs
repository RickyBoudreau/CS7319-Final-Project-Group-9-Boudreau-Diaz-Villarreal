use crate::Selected::sensor_frame::SensorFrame;
use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

pub const SIMULATE_REALTIME: bool = true;  // Set to true for real-time simulation

pub struct Selected {
    heart_rate: Vec<f64>,
    blood_pressure: Vec<String>,
    barometric_pressure: Vec<f64>,
    distance_traveled: Vec<f64>,
    steps_taken: Vec<i64>,
    time: Vec<f64>,
    temperature_outside: Vec<f64>,
    bluetooth_transmissions: Vec<String>,
    water_in_device: bool,
    water_removed_before: bool,
    // New power fields
    total_power: i32,
    power_consumption_per_second: i32,
}

impl Selected {
    pub fn new() -> Self {
        Self {
            heart_rate: Vec::new(),
            blood_pressure: Vec::new(),
            barometric_pressure: Vec::new(),
            distance_traveled: Vec::new(),
            steps_taken: Vec::new(),
            time: Vec::new(),
            temperature_outside: Vec::new(),
            bluetooth_transmissions: Vec::new(),
            water_in_device: false,
            water_removed_before: false,
            total_power: 1000,
            power_consumption_per_second: 9, // 9 sensors * 1 unit of power consumed per second each
        }
    }

    pub fn ingest_queue(&mut self, queue: &mut VecDeque<SensorFrame>) {
        while let Some(frame) = queue.pop_front() {
            self.apply_frame(frame);

            if SIMULATE_REALTIME {
                sleep(Duration::from_secs(1));
            }
        }
    }

    // Changed from private `fn` to `pub fn`
    pub fn apply_frame(&mut self, frame: SensorFrame) {
        self.heart_rate.push(frame.heart_rate);
        self.blood_pressure.push(frame.blood_pressure);
        self.barometric_pressure.push(frame.barometric_pressure);
        self.distance_traveled.push(frame.distance_traveled);
        self.steps_taken.push(frame.steps_taken);
        self.time.push(frame.time);
        self.temperature_outside.push(frame.temperature_outside);
        self.bluetooth_transmissions.push(frame.bluetooth_transmissions);

        if !self.water_removed_before {
            self.water_in_device = frame.water_in_device;
        }

        // Consume power for this second (never go below 0)
        self.total_power -= self.power_consumption_per_second;
        if self.total_power < 0 {
            self.total_power = 0;
        }
    }

    pub fn clear_device_water(&mut self) {
        self.water_in_device = false;
        self.water_removed_before = true;
    }

    pub fn get_heart_rate(&self) -> Option<f64> {
        self.heart_rate.last().copied()
    }

    pub fn get_blood_pressure(&self) -> Option<String> {
        self.blood_pressure.last().cloned()
    }

    pub fn get_barometric_pressure(&self) -> Option<f64> {
        self.barometric_pressure.last().copied()
    }

    pub fn get_time(&self) -> Option<f64> {
        self.time.last().copied()
    }

    pub fn get_temperature_outside(&self) -> Option<f64> {
        self.temperature_outside.last().copied()
    }

    pub fn get_water_in_device(&self) -> bool {
        self.water_in_device
    }

    pub fn get_distance_traveled(&self) -> f64 {
        self.distance_traveled.iter().sum()
    }

    pub fn get_steps_taken(&self) -> i64 {
        self.steps_taken.iter().sum()
    }

    pub fn get_bluetooth_transmissions(&self) -> Vec<String> {
        self.bluetooth_transmissions
            .iter()
            .filter(|s| s.as_str() != "NONE")
            .cloned()
            .collect()
    }

    // New power method
    pub fn get_power_level(&self) -> i32 {
        self.total_power
    }

    // New "get_all" methods returning references to the entire vectors
    pub fn get_all_heart_rate(&self) -> &Vec<f64> {
        &self.heart_rate
    }

    pub fn get_all_blood_pressure(&self) -> &Vec<String> {
        &self.blood_pressure
    }

    pub fn get_all_barometric_pressure(&self) -> &Vec<f64> {
        &self.barometric_pressure
    }

    pub fn get_all_time(&self) -> &Vec<f64> {
        &self.time
    }

    pub fn get_all_temperature_outside(&self) -> &Vec<f64> {
        &self.temperature_outside
    }
}