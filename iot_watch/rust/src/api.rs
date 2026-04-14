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
    let mut queue = load_sensor_queue("C:\\CS7319-Final-Project\\iot_watch\\rust\\src\\sensor_data.json")
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
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        rt.block_on(async move {
            let bus = EventBus::new();
            
            // 1. THE PATH FIX: Use the exact absolute path just like Blackboard
            let absolute_path = "C:\\CS7319-Final-Project\\iot_watch\\rust\\sensor_data.json";
            
            println!("Tokio: Attempting to load JSON from absolute path...");
            let loader = Arc::new(Mutex::new(EventSensorLoader::new(absolute_path)));
            println!("Tokio: JSON loaded successfully! Starting Sensor Manager...");

            let bus_clone = bus.clone();
            tokio::spawn(async move {
                crate::event_driven::sensor_manager::run_sensor_manager(bus_clone, loader).await;
            });

            let client = Client::new(bus.clone());
            client.open_health();
            client.open_weather();
            client.open_messages();
            client.open_water();

            let mut rx = bus.subscribe();
            
            let mut hr = "--".to_string();
            let mut bp = "--/--".to_string();
            let mut steps_val = 0;
            let mut dist_val = 0.0;
            let mut baro = "--".to_string();
            let mut temp = "--".to_string();
            let mut water = false;
            let mut msg: Option<String> = None;

            let mut interval = tokio::time::interval(Duration::from_secs(1));

            loop {
                tokio::select! {
                    Ok(event) = rx.recv() => {
                        // 2. THE HEARTBEAT FIX: Print every event that crosses the bus
                        // println!("Tokio Event Received: {:?}", event); 
                        // (You can uncomment the line above to see EVERYTHING, but it will be very spammy!)
                        
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
                    
                    _ = interval.tick() => {
                        // 3. PUSH CONFIRMATION
                        println!("Tokio: Packaging state and pushing to Flutter UI...");
                        
                        let ui_state = WatchUiState {
                            heart_rate: hr.clone(),
                            blood_pressure: bp.clone(),
                            steps: steps_val.to_string(),
                            distance: format!("~{:.0}", dist_val * 3.28084),
                            barometric_pressure: baro.clone(),
                            temperature: temp.clone(),
                            water_in_device: water,
                            latest_message: msg.clone(),
                        };
                        
                        let _ = sink.add(ui_state);
                    }
                }
            }
        });
    });

    Ok(())
}