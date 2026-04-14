use crate::frb_generated::StreamSink;
use std::thread;
use std::time::Duration;

// Blackboard Imports
use crate::blackboard::blackboard::Blackboard;
use crate::blackboard::sensor_loader::load_sensor_queue;

// Event Driven Imports (Adding these fixes the 'EventBus' and 'SensorLoader' undeclared errors)
use crate::event_driven::event_bus::EventBus;
use crate::event_driven::sensor_loader::SensorLoader as EventSensorLoader;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::event_driven::client::Client;
use crate::event_driven::event::Event;
use crate::event_driven::sensors::Sensors;

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
    thread::spawn(move || {
        // 1. The Event-Driven architecture requires a Tokio async runtime to work
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        rt.block_on(async move {
            // Initialize the Bus and the Event-specific Loader
            let bus = EventBus::new();
            let loader = Arc::new(Mutex::new(EventSensorLoader::new("sensor_data.json")));

            // 2. Spawn the backend sensor manager in the background
            let bus_clone = bus.clone();
            tokio::spawn(async move {
                crate::event_driven::sensor_manager::run_sensor_manager(bus_clone, loader).await;
            });

            // 3. Simulate the user opening all the apps! 
            // In your friend's architecture, sensors stay off to save power unless an app asks for them.
            let client = Client::new(bus.clone());
            client.open_health();
            client.open_weather();
            client.open_messages();
            client.open_water();

            // 4. Subscribe to the Event Bus to catch the data meant for the Flutter UI
            let mut rx = bus.subscribe();
            
            // Local state cache to hold the latest values as events come in
            let mut hr = "--".to_string();
            let mut bp = "--/--".to_string();
            let mut steps_val = 0;
            let mut dist_val = 0.0;
            let mut baro = "--".to_string();
            let mut temp = "--".to_string();
            let mut water = false;
            let mut msg: Option<String> = None;

            // Timer to push the packaged UI state to Flutter every second
            let mut interval = tokio::time::interval(Duration::from_secs(1));

            loop {
                tokio::select! {
                    // Whenever an event comes through the bus, update the local cache
                    Ok(event) = rx.recv() => {
                        if let Event::SensorData(sensor) = event {
                            match sensor {
                                Sensors::HeartRate(v) => hr = format!("{:.0}", v),
                                Sensors::BloodPressure(v) => bp = v,
                                Sensors::Steps(v) => steps_val += v, 
                                Sensors::DistanceTraveled(v) => dist_val += v, 
                                Sensors::BarometricPressure(v) => baro = format!("{:.2}", v),
                                Sensors::Temperature(v) => temp = format!("{:.1}", v),
                                Sensors::WaterDetected(v) => water = v,
                                Sensors::Bluetooth(v) => {
                                    if v != "NONE" {
                                        msg = Some(v);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    // Every second, package the cache and send it across the bridge to Flutter
                    _ = interval.tick() => {
                        let ui_state = WatchUiState {
                            heart_rate: hr.clone(),
                            blood_pressure: bp.clone(),
                            steps: steps_val.to_string(),
                            distance: format!("~{:.0}", dist_val * 3.28084), // Convert to feet
                            barometric_pressure: baro.clone(),
                            temperature: temp.clone(),
                            water_in_device: water,
                            latest_message: msg.clone(),
                        };
                        
                        // Send state across the FFI boundary
                        let _ = sink.add(ui_state);
                    }
                }
            }
        });
    });

    Ok(())
}