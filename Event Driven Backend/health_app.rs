use crate::event::Event;
use crate::event_bus::EventBus;
use crate::sensors::Sensors;

pub async fn run_health_app(bus: EventBus) {
    let mut rx = bus.subscribe();

    let mut total_steps: i64 = 0;
    let mut total_distance: f64 = 0.0;
    let mut heart_rate: Option<f64> = None;
    let mut blood_pressure: Option<String> = None;
    let mut time: Option<f64> = None;
    let mut active = false;

    loop {
        if let Ok(event) = rx.recv().await {
            match event {
                Event::RequestHealth => {
                    if !active {
                        active = true;
                        println!("User opened Weather App");
                        bus.publish(Event::RequestSensor("Heart Rate".into()));
                        bus.publish(Event::RequestSensor("Blood Pressure".into()));
                        bus.publish(Event::RequestSensor("Steps Taken".into()));
                        bus.publish(Event::RequestSensor("Distance Traveled".into()));
                        bus.publish(Event::RequestSensor("Time".into()));
                    }
                }

                Event::StopHealth => {
                    if active {
                        active = false;
                        println!("User closed Weather App");
                        bus.publish(Event::StopSensor("Heart Rate".into()));
                        bus.publish(Event::StopSensor("Blood Pressure".into()));
                        bus.publish(Event::StopSensor("Steps Taken".into()));
                        bus.publish(Event::StopSensor("Distance Traveled".into()));
                        bus.publish(Event::StopSensor("Time".into()));
                    }
                }

                Event::SensorData(sensor) => if active {
                    match sensor {
                        Sensors::HeartRate(v) => heart_rate = Some(v),
                        Sensors::BloodPressure(v) => blood_pressure = Some(v),
                        Sensors::Steps(v) => total_steps += v,
                        Sensors::DistanceTraveled(v) => total_distance += v,
                        Sensors::Time(v) => time = Some(v),
                        _ => {}
                    }

                    println!("Health App displayed: Heart Rate: {:?} bpm, Blood Pressure: {:?}, Steps taken (total): {:?}, Distance traveled (total): {:?} m, Time: {:?} s",
                             heart_rate, blood_pressure, total_steps, total_distance, time
                    );
                }


                _ => {}
            }
        }
    }
}