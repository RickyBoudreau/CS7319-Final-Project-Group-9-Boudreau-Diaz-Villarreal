use crate::event::Event;
use crate::event_bus::EventBus;

pub struct Client {
    bus: EventBus,
}

impl Client {
    pub fn new(bus: EventBus) -> Self {
        Self { bus }
    }

    pub fn open_health(&self) {
        self.bus.publish(Event::RequestHealth);
    }

    pub fn close_health(&self) {
        self.bus.publish(Event::StopHealth);
    }

    pub fn open_weather(&self) {
        self.bus.publish(Event::RequestWeather);
    }

    pub fn close_weather(&self) {
        self.bus.publish(Event::StopWeather);
    }

    pub fn open_messages(&self) {
        self.bus.publish(Event::RequestMessages);
    }

    pub fn close_messages(&self) {
        self.bus.publish(Event::StopMessages);
    }

    pub fn open_water(&self) {
        self.bus.publish(Event::TriggerWaterRemoval);
    }

    pub fn close_water(&self) {
        self.bus.publish(Event::StopWaterRemoval);
    }

    pub fn clear_water(&self) {
        self.bus.publish(Event::StartWaterRemoval);
    }
}