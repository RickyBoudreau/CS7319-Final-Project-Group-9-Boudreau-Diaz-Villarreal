use tokio::sync::broadcast;
use crate::Unselected::event::Event;

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(300);
        Self { sender }
    }

    //Sensors call this to publish their data
    pub fn publish(&self, event: Event) {
        let _ = self.sender.send(event);
    }

    //Apps call this to start listening for events
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}

