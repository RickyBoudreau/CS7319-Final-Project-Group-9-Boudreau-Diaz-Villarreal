use std::sync::Arc;
use tokio::sync::Mutex;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;
use crate::frb_generated::StreamSink;

// Selected Imports
use crate::Selected::Selected::Blackboard;
use crate::Selected::sensor_loader::load_sensor_queue;

// Event Driven Imports
use crate::Unselected::event_bus::EventBus;
use crate::Unselected::sensor_loader::SensorLoader as EventSensorLoader;
use crate::Unselected::client::Client;
use crate::Unselected::event::Event;
use crate::Unselected::sensors::Sensors;

// Create a global lock for the EventBus
static GLOBAL_BUS: OnceLock<EventBus> = OnceLock::new();

// The Unified UI State
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

pub fn notify_app_opened(app_id: String) {
    if let Some(bus) = GLOBAL_BUS.get() {
        let client = Client::new(bus.clone());
        match app_id.as_str() {
            "health" => client.open_health(),
            "weather" => client.open_weather(),
            "messages" => client.open_messages(),
            "water" => client.open_water(),
            _ => {}
        }
    }
}

pub fn notify_app_closed(app_id: String) {
    if let Some(bus) = GLOBAL_BUS.get() {
        let client = Client::new(bus.clone());
        match app_id.as_str() {
            "health" => client.close_health(),
            "weather" => client.close_weather(),
            "messages" => client.close_messages(),
            "water" => client.close_water(),
            _ => {}
        }
    }
}
// 2. ARCHITECTURE A: Selected ENTRY POINT
pub fn start_blackboard_simulation(sink: StreamSink<WatchUiState>) -> anyhow::Result<()> {
    let mut queue = load_sensor_queue("C:\\CS7319-Final-Project\\iot_watch\\rust\\src\\sensor_data.json")
        .map_err(|e| anyhow::anyhow!("Failed to load data: {}", e))?;

    thread::spawn(move || {
        let mut Selected = Selected::new();
        // ... (Your while loop logic here) ...
        while let Some(frame) = queue.pop_front() {
            Selected.apply_frame(frame);
            
            // Map Selected state to WatchUiState
            let ui_state = WatchUiState {
                heart_rate: Selected.get_heart_rate().map(|v| format!("{:.0}", v)).unwrap_or_else(|| "--".to_string()),
                blood_pressure: Selected.get_blood_pressure().unwrap_or_else(|| "--/--".to_string()),
                steps: Selected.get_steps_taken().to_string(),
                distance: format!("~{:.0}", Selected.get_distance_traveled() * 3.28084),
                barometric_pressure: Selected.get_barometric_pressure().map(|v| format!("{:.0}", v)).unwrap_or_else(|| "--".to_string()),
                temperature: Selected.get_temperature_outside().map(|v| format!("{:.0}", v)).unwrap_or_else(|| "--".to_string()),
                water_in_device: Selected.get_water_in_device(),
                latest_message: Selected.get_bluetooth_transmissions().last().cloned(),
            };

            sink.add(ui_state);
            thread::sleep(Duration::from_secs(1));
        }
    });
    Ok(())
}

// 3. ARCHITECTURE B: EVENT-DRIVEN ENTRY POINT
pub fn start_event_driven_simulation(sink: StreamSink<WatchUiState>) -> anyhow::Result<()> {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        rt.block_on(async move {
            let bus = EventBus::new();
            
            // Store the bus globally so `notify_app_opened` can access it!
            let _ = GLOBAL_BUS.set(bus.clone()); 
            
            let absolute_path = "C:\\CS7319-Final-Project\\iot_watch\\rust\\src\\sensor_data.json";
            let loader = Arc::new(Mutex::new(EventSensorLoader::new(absolute_path)));

            let bus_clone = bus.clone();
            tokio::spawn(async move {
                crate::Unselected::sensor_manager::run_sensor_manager(bus_clone, loader).await;
            });

            let bus_health = bus.clone();
            tokio::spawn(async move {
                crate::Unselected::health_app::run_health_app(bus_health).await;
            });

            let bus_weather = bus.clone();
            tokio::spawn(async move {
                crate::Unselected::weather_app::run_weather_app(bus_weather).await;
            });

            let bus_msg = bus.clone();
            tokio::spawn(async move {
                crate::Unselected::message_app::run_message_app(bus_msg).await;
            });

            let bus_water = bus.clone();
            tokio::spawn(async move {
                crate::Unselected::water_removal_app::run_water_removal_app(bus_water).await;
            });

            let mut rx = bus.subscribe();
            
            let mut hr = "--".to_string();
            let mut bp = "--/--".to_string();
            let mut steps_val = 0;
            let mut dist_val = 0.0;
            let mut baro = "--".to_string();
            let mut temp = "--".to_string();
            let mut water = false;
            let mut msg: Option<String> = None;

            let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

            loop {
                tokio::select! {
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
                    
                    _ = interval.tick() => {
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