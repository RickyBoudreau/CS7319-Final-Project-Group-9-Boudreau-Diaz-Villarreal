use crate::blackboard::Blackboard;
use crate::sensor_loader::load_sensor_queue;
use flutter_rust_bridge::StreamSink;
use std::thread;
use std::time::Duration;

// This is the data packet that gets sent to Flutter every second.
// flutter_rust_bridge will automatically generate a matching Dart class for this.
#[derive(Clone, Debug)]
pub struct BlackboardUiState {
    pub heart_rate: String,
    pub blood_pressure: String,
    pub steps: String,
    pub distance: String,
    pub barometric_pressure: String,
    pub temperature: String,
    pub water_in_device: bool,
    pub latest_message: Option<String>,
}

// This function is called ONCE from Flutter to start the simulation loop.
pub fn start_blackboard_simulation(sink: StreamSink<BlackboardUiState>) -> anyhow::Result<()> {
    // 1. Load the queue (handling the error safely for the FFI boundary)
    let mut queue = load_sensor_queue("sensor_data.json")
        .map_err(|e| anyhow::anyhow!("Failed to load sensor data: {}", e))?;

    // 2. Spawn the background thread so we don't freeze the Flutter UI
    thread::spawn(move || {
        let mut blackboard = Blackboard::new();
        let mut second_count = 0;

        // 3. The main simulation loop
        while let Some(frame) = queue.pop_front() {
            second_count += 1;
            blackboard.apply_frame(frame);

            // Simulate the user clearing water at 30 seconds
            if second_count == 30 {
                blackboard.clear_device_water();
            }

            // Get the most recent message, if any exist
            let all_messages = blackboard.get_bluetooth_transmissions();
            let latest_msg = all_messages.last().cloned();

            // Convert meters to feet for the UI (1 meter = 3.28084 feet)
            let distance_ft = blackboard.get_distance_traveled() * 3.28084;

            // 4. Package the current state of the Blackboard
            let ui_state = BlackboardUiState {
                heart_rate: blackboard.get_heart_rate().map(|v| format!("{:.0}", v)).unwrap_or_else(|| "--".to_string()),
                blood_pressure: blackboard.get_blood_pressure().unwrap_or_else(|| "--/--".to_string()),
                steps: blackboard.get_steps_taken().to_string(),
                distance: format!("~{:.0}", distance_ft),
                barometric_pressure: blackboard.get_barometric_pressure().map(|v| format!("{:.0}", v)).unwrap_or_else(|| "--".to_string()),
                temperature: blackboard.get_temperature_outside().map(|v| format!("{:.0}", v)).unwrap_or_else(|| "--".to_string()),
                water_in_device: blackboard.get_water_in_device(),
                latest_message: latest_msg,
            };

            // 5. Send the state to Flutter!
            sink.add(ui_state);

            // Wait 1 second before processing the next frame
            thread::sleep(Duration::from_secs(1));
        }
    });

    Ok(())
}