use tokio::sync:: Mutex;
use std::sync::Arc;
use std::time::Duration;

mod sensors;
mod event_bus;
pub mod client;
pub mod sensor_loader;
pub mod sensor_manager;
pub mod event;
pub mod health_app;
pub mod weather_app;
pub mod message_app;
pub mod water_removal_app;

use event_bus::EventBus;
use crate::client::Client;
use crate::sensor_loader::SensorLoader;
use crate::health_app::run_health_app;
use crate::message_app::run_message_app;
use crate::sensor_manager::run_sensor_manager;
use crate::water_removal_app::run_water_removal_app;
use crate::weather_app::run_weather_app;

#[tokio::main]
async fn main() {
    let bus = EventBus::new();
    let loader = Arc::new(Mutex::new(SensorLoader::new("sensor_data.json")));

    tokio::spawn(run_sensor_manager(bus.clone(), loader.clone()));
    tokio::spawn(run_health_app(bus.clone()));
    tokio::spawn(run_weather_app(bus.clone()));
    tokio::spawn(run_message_app(bus.clone()));
    tokio::spawn(run_water_removal_app(bus.clone()));

    tokio::time::sleep(Duration::from_millis(100)).await;

    let ui = Client::new(bus.clone());

    // Simulate user opening/closing apps
    ui.open_messages();
    tokio::time::sleep(Duration::from_secs(6)).await;
    ui.close_messages();
    tokio::time::sleep(Duration::from_millis(100)).await;

    ui.open_health();
    tokio::time::sleep(Duration::from_secs(5)).await;
    ui.close_health();
    tokio::time::sleep(Duration::from_millis(100)).await;

    ui.open_weather();
    tokio::time::sleep(Duration::from_secs(5)).await;
    ui.close_weather();
    tokio::time::sleep(Duration::from_millis(100)).await;

    ui.open_water();
    tokio::time::sleep(Duration::from_secs(5)).await;
    ui.clear_water();
    tokio::time::sleep(Duration::from_secs(3)).await;
    ui.close_water();
    tokio::time::sleep(Duration::from_millis(100)).await;

    tokio::signal::ctrl_c().await.unwrap();
}