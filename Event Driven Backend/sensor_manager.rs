use std::collections::HashSet;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use crate::event::Event;
use crate::event_bus::EventBus;
use crate::sensor_loader::SensorLoader;

pub async fn run_sensor_manager(bus: EventBus, loader: Arc<Mutex<SensorLoader>>) {
    let mut rx = bus.subscribe();
    //only publish active sensors
    let mut active_sensors: HashSet<String> = HashSet::new();
    let mut total_power: i32 = 1000;

    loop {
        while let Ok(event) = rx.try_recv() {
            match event {
                Event::RequestSensor(name) => {
                    println!("Backend requesting sensor {}", name);
                    active_sensors.insert(name);
                }
                Event::StopSensor(name) => {
                    println!("Backend stopping sensor {}", name);
                    active_sensors.remove(&name);
                }
                _ => {}
            }
        }

        if !active_sensors.is_empty() {
            let mut loader = loader.lock().await;
            let snapshot = loader.next_snapshot();

            for sensor in snapshot {
                if active_sensors.contains(sensor.name()) {
                    println!("Publishing {:?}", sensor);
                    bus.publish(Event::SensorData(sensor));
                }
            }

            let power_consumption_per_second = active_sensors.len() as i32;
            total_power -= power_consumption_per_second;

            if total_power < 0 {
                total_power = 0;
            }
            println!("Remaining total power: {}", total_power);
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}