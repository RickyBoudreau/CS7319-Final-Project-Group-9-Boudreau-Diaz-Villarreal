use crate::event_driven::event::Event;
use crate::event_driven::event_bus::EventBus;
use crate::event_driven::sensors::Sensors;

pub async fn run_message_app(bus: EventBus) {
    let mut rx = bus.subscribe();
    let mut active = false;
    let mut messages: Vec<String> = Vec::new();

    loop {
        if let Ok(event) = rx.recv().await {
            match event {
                Event::RequestMessages => {
                    if !active {
                        active = true;
                        println!("User opened Messages App");
                        bus.publish(Event::RequestSensor("Bluetooth Transmissions".into()));
                    }
                }

                Event::StopMessages => {
                    if active {
                        active = false;
                        println!("User closed Messages App");
                        bus.publish(Event::StopSensor("Bluetooth Transmissions".into()));
                    }
                }

                Event::SensorData(sensor) if active => {
                    if let Sensors::Bluetooth(v) = sensor {
                        if v != "NONE" {
                            messages.push(v.clone());
                        }
                        println!("Messages App displayed Messages: {:?}", messages);
                    }
                }

                _ => {}
            }
        }
    }
}