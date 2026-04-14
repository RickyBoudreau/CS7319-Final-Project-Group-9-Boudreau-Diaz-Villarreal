use crate::sensors::Sensors;

#[derive(Debug, Clone)]
pub enum Event {
    RequestHealth,
    StopHealth,
    RequestWeather,
    StopWeather,
    RequestMessages,
    StopMessages,
    RequestWaterRemoval,
    StopWaterRemoval,
    ClearWater,
    RequestSensor(String),
    StopSensor(String),
    SensorData(Sensors)
}