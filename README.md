# Blackboard vs. Event-Driven Design for IOT Wearables
This repository serves as a means to compare two different architectural styles, namely a "blackboard" design and an "event-driven" design, and their practicality in real-world deployments on Internet of Things (IOT) wearable devices. For our simulation environment, we have created a synthetic smart watch, whose backend has been implemented twice, once per architectural style. This watch is responsible for capturing a wide variety of information about the environment and user's health via 9 sensors, which are controlled in different ways depending on the underlying architecture's capabilities, and then displaying that information to the user.

## Note About the Water Sensor
In order to effectively test our system's ability to detect and remove simulated water, while maintainging our human-readible JSON raw data format, the JSON data will continue to show the device as being water logged even after it has been cleared. In other words, for the sake of our simulation, where we cannot know the arbitrary time T in which the user will dispell the water, the JSON file of raw sensor data will continue to send a waterlogged signal to the IOT wearable; the wearable itself will simply ignore this sensor's waterlogged signal if water has been cleared at least one time during the simulation. In the real world, where the data is not fixed to a JSON file and events are truly random (i.e., they do not need to be deterministic for the sake of our clean/clear demonstration), this manipulation of the waterlogged flag would be removed, and instead the system would simply purge/detect water as expected.

## Brief Overview of Styles
While we discuss more in-depth the differences between our styles below, this section provides a quick-start to understanding what can be found in the code/implementation of each architecture.

**Blackboard**
- Central data store
- Constant polling of sensors
- Longer-term "memory" in comparison to event-driven
- Higher simulated power consumption in comparison to event-driven

**Event-Driven**
- No centralized data store
- Sensors are polled only when user apps explicitly requires it
- Shorter-term "memory" in comparison to blackboard
- Lower simuated power consumption in comparison to blackboard


## Compilation and Implementation Instructions
### Compilation and Implementation Platform
TODO
### How to Compile Our Code
TODO
### How to Execute Our Code
TODO

## Detailed Explanation of Differences in the Architectural Styles Selected
### Blackboard Implementation
Our blackboard architecture is implemented as a centralized and mutable data repository. The ```Blackboard``` struct stores every sensor reading in per-sensor vectors (in the format ```heart_rate: Vec<f64>```, etc.). This core pattern of interaction allows for a continuous accumulation of data each second -- as simulated data is "read" by the sensors from the JSON file (then place into a ```SensorFrame```), each incoming ```SensorFrame``` is applied via ```apply_frame()```, pushing each of the fields into its own respetive vector within the blackboard. At the same time, the blackboard also updates the boolean flag for water intrusion and deducts a fixed power cost per sensor polled. By designing the system in this way, it is able to make historial data into a first-class citizen, where the blackboard can retain the entire slate of data from the current runtime (as well as access it via the ```get_all_*()``` functions). As a result, all components -- such as a downstream analystics module or the UI -- can easily and on-demand retrieve not only the most recently recorded value but also the complete trend.

Similarly, the connectors between the blackboard and the sensors is direct and implicit. The controlling class pops frames from a queue and synchronously calls ```apply_frame()```, such that there is not a truly separate abstraction of the connector(s) (i.e., there is not event bus or message router). By coupling these features tightly together, the blackboard is able to demonstrate its unique features and benefits -- the blackboard itself becomes the sole intermediary, with all aspects of communication flowing to and from it via simply, ordinary function calls.

### Event-Driven Implementation
However, in contrast to the blackboard implementation, the event-driven design... TODO (rubric wants a lot of talk about specifics about the actual code)

## Detailed Explanation of Rationale Behind Final Architecture Selection
After careful review of the data gathered during development and experimentation with our system, we have determined the blackboard architecture to be the most effective at meeting our IOT wearable's real-world needs. 
This is because...
TODO
