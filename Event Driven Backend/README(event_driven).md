# README: Event Driven Backend Architecture

This code includes everything needed to run and simulate an event driven architecture for this simulated IOT device.
The difference from the Blackboard style is that the sensors are
reactive and are only polling for data when a user interacts
with an application. Sensors "sleep" when not in use and only
produce data when the application requests it.
This design mocks how a real IOT wearable drive system would
subscribe to sensors to produce data only when needed, which
conserves battery life of the device.

In this code, there are 5 main components that support the communication
between the simulated apps and sensors.
These are the EventBus, SensorManager, Applications (simulated),
Client and SensorLoader.
(event_bus.rs, sensor_manager.rs, client.rs, sensor_loader.rs
weather_app.rs, health_app.rs, message_app.rs, water_removal_app.rs)

- The EventBus is the central point of communication. It is used as a
dispatcher, publishing and broadcasting events to subscribers.
Here, the applications are the subscribers and the sensors
are the publishers.
- The SensorManager manages all communication related to the sensors.
It triggers or starts the sensors to be active when the user
interacts with the related app, loads the JSON simulated data
snapshots, and publishes that loaded JSON sensor data.
- The Client is an api for the UI. It calls the appropriate high level functions like opens and closing apps like a user would and triggers the clear water
button.
- The SensorLoader reads 1 second of data from the JSON file
at a time (this depends on when the sensor is triggered).
- The apps include the WeatherApp, MessageApp, HealthApp and
WaterRemovalApp. 
  - The Water Removal App requests the data from 2 sensors
  - The Health App requests data from 5 sensors
  - The Message App requests data from 1 sensor
  - The Weather App requests data from 1 sensor

Each app maintains its own temporary state while it is open to set the relative sensors to be active. It also has its own defined variables to track water being cleared, total steps, total distance and all messages.
- Health: total (cumulative) steps, total (cumulative) distance, heart rate, blood pressure and time
- Weather: temperature and barometric pressure
- Messages: all Bluetooth messages
- Water Removal: simulated water flag with "cleared once" behavior

# File hierarchy
Event Driven Backend
- src/
  - main.rs
  - event.rs 
  - sensors.rs 
  - event_bus.rs 
  - sensor_loader.rs 
  - sensor_manager.rs 
  - client.rs 
  - weather_app.rs 
  - message_app.rs 
  - health_app.rs 
  - water_removal_app.rs
- sensor_data.json
- Cargo.toml

# Testing
When the file hierarchy is correct, to run use `cargo run`
to begin the simulation for the backend. An example of the simulated
output is included in a separate png.

# API Access
The Event driven backend does not expose getter functions
or shared memory. 

The UI interacts via Events that are published
to the EventBus. App behavior is then triggered by calling the functions
from the Client. (ex: open_health, close_health, etc.) 

Each app function
is activated in the Client, and they publish events when called on by
the UI. (ex: the open_health() call publishes the RequestHealth event to
activate the related 5 health app sensors: Heart rate, Barometric Pressure,
Steps taken, Distance traveled, and Time. the close_health() call
then publishes the StopHealth event to stop the activity of those
5 sensors.) 

This ensures each sensor is only active when a user opens an
app, and they can be unactive again if no call from the UI comes.
The UI listens for the SensorData(Sensors) event response with the name of the
sensor value and updates the display when a new SensorData event is
received.
