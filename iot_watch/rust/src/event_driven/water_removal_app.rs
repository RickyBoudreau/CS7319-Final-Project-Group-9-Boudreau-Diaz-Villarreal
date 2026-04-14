use crate::event_driven::event::Event;
use crate::event_driven::event_bus::EventBus;
use crate::event_driven::sensors::Sensors;

pub async fn run_water_removal_app(bus: EventBus) {
    let mut rx = bus.subscribe();
    let mut active = false;

    let mut water_present: bool = false;
    let mut water_cleared: bool = false;

    loop {
        if let Ok(event) = rx.recv().await {
            match event {
                Event::RequestWaterRemoval => {
                    if !active {
                        active = true;
                        println!("User opened Water Removal App");
                        bus.publish(Event::RequestSensor("Water in Device".into()));
                    }
                }

                Event::StopWaterRemoval => {
                    if active {
                        active = false;
                        println!("User closed Water Removal App");
                        bus.publish(Event::StopSensor("Water in Device".into()));
                    }
                }

                Event::SensorData(sensor) if active => {
                    if let Sensors::WaterDetected(v) = sensor {
                        if !water_cleared {
                            water_present = v;
                        } else {
                            water_present = false;
                        }

                        println!(
                            "Water App displays Water present in device: {}",
                            water_present
                        );
                    }
                }

                Event::ClearWater if active => {
                    println!("Water cleared from device");
                    water_present = false;
                    water_cleared = true;
                }

                _ => {}
            }
        }
    }
}