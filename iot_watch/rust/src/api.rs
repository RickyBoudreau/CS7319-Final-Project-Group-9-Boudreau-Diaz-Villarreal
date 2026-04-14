// rust/src/api.rs
use flutter_rust_bridge::StreamSink;
use std::thread;
use std::time::Duration;

// Import shared loader and both architectures
use crate::shared_models::sensor_loader::load_sensor_queue;
use crate::blackboard::blackboard_core::Blackboard;
use crate::event_driven::event_bus::EventBus;

// 1. THE UNIFIED DATA STRUCT
// Both architectures will populate this exact same struct to send to Flutter.
#[derive(Clone, Debug)]
pub struct WatchUiState {
    pub heart_rate: String,
    pub blood_pressure: String,
    pub steps: String,
    pub distance: String,
    pub barometric_pressure: String,
    pub temperature: String,
    pub water_in_device: bool,
    pub latest_message: Option<String>,
}

// 2. ARCHITECTURE A: BLACKBOARD ENTRY POINT
pub fn start_blackboard_simulation(sink: StreamSink<WatchUiState>) -> anyhow::Result<()> {
    let mut queue = load_sensor_queue("sensor_data.json")
        .map_err(|e| anyhow::anyhow!("Failed to load data: {}", e))?;

    thread::spawn(move || {
        let mut blackboard = Blackboard::new();
        // ... (Your while loop logic here) ...
        while let Some(frame) = queue.pop_front() {
            blackboard.apply_frame(frame);
            
            // Map Blackboard state to WatchUiState
            let ui_state = WatchUiState {
                heart_rate: blackboard.get_heart_rate().map(|v| format!("{:.0}", v)).unwrap_or_else(|| "--".to_string()),
                blood_pressure: blackboard.get_blood_pressure().unwrap_or_else(|| "--/--".to_string()),
                steps: blackboard.get_steps_taken().to_string(),
                distance: format!("~{:.0}", blackboard.get_distance_traveled() * 3.28084),
                barometric_pressure: blackboard.get_barometric_pressure().map(|v| format!("{:.0}", v)).unwrap_or_else(|| "--".to_string()),
                temperature: blackboard.get_temperature_outside().map(|v| format!("{:.0}", v)).unwrap_or_else(|| "--".to_string()),
                water_in_device: blackboard.get_water_in_device(),
                latest_message: blackboard.get_bluetooth_transmissions().last().cloned(),
            };

            sink.add(ui_state);
            thread::sleep(Duration::from_secs(1));
        }
    });
    Ok(())
}

// 3. ARCHITECTURE B: EVENT-DRIVEN ENTRY POINT
pub fn start_event_driven_simulation(sink: StreamSink<WatchUiState>) -> anyhow::Result<()> {
    let mut queue = load_sensor_queue("sensor_data.json")
        .map_err(|e| anyhow::anyhow!("Failed to load data: {}", e))?;

    thread::spawn(move || {
        // Initialize your Event Bus and Subscribers here
        let mut event_bus = EventBus::new();
        
        while let Some(frame) = queue.pop_front() {
            // Publish frame data to the event bus
            event_bus.publish_frame(frame);
            
            // 2. Query your localized states/subscribers
            let hr_state = event_bus.get_subscriber("heart_rate").get_value();
            
            // 3. Map the Event state to the EXACT SAME WatchUiState
            let ui_state = WatchUiState {
                heart_rate: "--".to_string(), // Replace with hr_state
                blood_pressure: "--/--".to_string(),
                steps: "--".to_string(),
                distance: "--".to_string(),
                barometric_pressure: "--".to_string(),
                temperature: "--".to_string(),
                water_in_device: false,
                latest_message: None,
            };

            sink.add(ui_state);
            thread::sleep(Duration::from_secs(1));
        }
    });
    Ok(())
}