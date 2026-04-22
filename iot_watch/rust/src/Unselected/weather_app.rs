use crate::Unselected::event::Event;
use crate::Unselected::event_bus::EventBus;
use crate::Unselected::sensors::Sensors;

pub async fn run_weather_app(bus: EventBus) {
    let mut rx = bus.subscribe();
    let mut active = false;

    let mut temperature_outside: Option<f64> = None;
    let mut baro_pressure: Option<f64> = None;

    loop {
        if let Ok(event) = rx.recv().await {
            match event {
                Event::RequestWeather => {
                    if !active {
                        active = true;
                        println!("User opened Weather App");
                        bus.publish(Event::RequestSensor("Temperature Outside".into()));
                        bus.publish(Event::RequestSensor("Barometric Pressure".into()));
                    }
                }

                Event::StopWeather => {
                    if active {
                        active = false;
                        println!("User closed Weather App");
                        bus.publish(Event::StopSensor("Temperature Outside".into()));
                        bus.publish(Event::StopSensor("Barometric Pressure".into()));
                    }
                }

                Event::SensorData(sensor) => if active{
                    match sensor {
                        Sensors::Temperature(v) => temperature_outside = Some(v),
                        Sensors::BarometricPressure(v) => baro_pressure = Some(v),
                        _ => {}
                    }

                    println!(
                        "Weather App displays Temperature: {:?} °C, Barometric Pressure: {:?} hPa",
                        temperature_outside, baro_pressure);
                },

                _ => {}
            }
        }
    }
}