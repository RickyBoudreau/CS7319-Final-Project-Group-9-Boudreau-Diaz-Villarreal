# Blackboard vs. Event-Driven Design for IOT Wearables
This repository serves as a means to compare two different architectural styles, namely a "blackboard" design and an "event-driven" design, and their practicality in real-world deployments on Internet of Things (IOT) wearable devices. For our simulation environment, we have created a synthetic smart watch, whose backend has been implemented twice, once per architectural style. This watch is responsible for capturing a wide variety of information about the environment and user's health via 9 sensors, which are controlled in different ways depending on the underlying architecture's capabilities, and then displaying that information to the user.

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
Blah blah Ricky writes here

### Event-Driven Implementation
However, in contrast to the blackboard implementation, the event-driven design... TODO (rubric wants a lot of talk about specifics about the actual code)

## Detailed Explanation of Rationale Behind Final Architecture Selection
After careful review of the data gathered during development and experimentation with our system, we have determined the blackboard architecture to be the most effective at meeting our IOT wearable's real-world needs. 
This is because...
TODO
