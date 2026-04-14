use crate::sensor_frame::SensorFrame;
use serde_json::Value;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

pub fn load_sensor_queue(path: &str) -> Result<VecDeque<SensorFrame>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let root: Value = serde_json::from_str(&text)?;

    let root_obj = root
        .as_object()
        .ok_or("Top-level JSON must be an object")?;

    let mut seconds: Vec<(usize, &Value)> = root_obj
        .iter()
        .filter_map(|(key, value)| key.parse::<usize>().ok().map(|n| (n, value)))
        .collect();

    seconds.sort_by_key(|(n, _)| *n);

    let mut queue = VecDeque::new();

    for (_, value) in seconds {
        queue.push_back(parse_one_second(value)?);
    }

    Ok(queue)
}

fn parse_one_second(value: &Value) -> Result<SensorFrame, Box<dyn Error>> {
    let mut frame = SensorFrame::default();

    let arr = value
        .as_array()
        .ok_or("Each second must map to an array")?;

    for item in arr {
        let obj = item
            .as_object()
            .ok_or("Each array item must be an object")?;

        for (sensor_name, sensor_value) in obj {
            match sensor_name.as_str() {
                "Heart Rate" => {
                    if let Some(v) = sensor_value.as_f64() {
                        frame.heart_rate = v;
                    }
                }
                "Blood Pressure" => {
                    if let Some(v) = sensor_value.as_str() {
                        frame.blood_pressure = v.to_string();
                    }
                }
                "Barometric Pressure" => {
                    if let Some(v) = sensor_value.as_f64() {
                        frame.barometric_pressure = v;
                    }
                }
                "Distance Traveled" => {
                    if let Some(v) = sensor_value.as_f64() {
                        frame.distance_traveled = v;
                    }
                }
                "Steps Taken" => {
                    if let Some(v) = sensor_value.as_i64() {
                        frame.steps_taken = v;
                    }
                }
                "Time" => {
                    if let Some(v) = sensor_value.as_f64() {
                        frame.time = v;
                    }
                }
                "Temperature Outside" => {
                    if let Some(v) = sensor_value.as_f64() {
                        frame.temperature_outside = v;
                    }
                }
                "Bluetooth Transmissions" => {
                    if let Some(v) = sensor_value.as_str() {
                        frame.bluetooth_transmissions = v.to_string();
                    }
                }
                "Water in Device" => {
                    if let Some(v) = sensor_value.as_bool() {
                        frame.water_in_device = v;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(frame)
}