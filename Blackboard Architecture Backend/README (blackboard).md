# Blackboard Architecture README
This rust crate contains the code necessary to create and test the blackboard architecture design for the
simulated IOT wearable. It ingests a predefined 60 "seconds" of watch sensor data from sensor_data.json and
exposes a rudimentary API of getter functions to access the ingested data. The data store is in the form of a
blackboard, with interactions all being posted or received centrally from this blackboard.

Namely, it ingests the data from the json file into a queue (ordered chronologically by the "second" in which
the data would be found by a real sensor). That data is then to be fed into a Blackboard object, whose values
can be read/written to as needed via the provided function API.

## Proper File Hierarchy
Information about the proper file hierarchy needed to run this version of the backend is found in "file hierarchy.txt".

## Testing
Once the file hierarchy is correct, use ```cargo run``` to begin the simulation. An example of correct simulated output
can be found in "example output.png".
- Note: if you wish to disable the "real time" nature of the simulation (i.e., you do not want the program
to ingest the simulated data at a rate of 1 real second per simulated second), then set "SIMULATE_REALTIME" to false
in blackboard.rs.

## API access
Create a a Blackboard object and ingest the data via the queue and a loop. Then, use the following functions for access:
```
    blackboard.get_heart_rate(),
    blackboard.get_blood_pressure(),
    blackboard.get_barometric_pressure(),
    blackboard.get_time(),
    blackboard.get_temperature_outside(),
    blackboard.get_water_in_device(),
    blackboard.get_distance_traveled(),
    blackboard.get_steps_taken(),
    blackboard.get_bluetooth_transmissions()
```
Likewise, the function "blackboard.clear_device_water()" can be called to clear the water and set the related internal water flags
to their appropriate values automatically.

Note: while these functions generally return the most recently observed sensor reading (i.e., if 34 seconds have passed, and
the get_heart_rate() function is called, the heart rate recorded at second 34 is returned, despite all recorded heart rates
being stored), the following functions have a unique mechanism to calculate their return value:
- get_water_in_device(): returns the most recent sensor data *unless* water has been cleared at least once before, in which case it returns false (while unrealistic, it highlights the intended use case more effectively in this simulation)
- get_distance_traveled(): returns the cumulative total distance traveled by the device wearer, not simply the amount of distance
traveled in the most recent second
- get_steps_taken(): returns the cumulative total steps taken by the device wearer, not simply the amount of steps taken in the
most recent second
- get_bluetooth_transmissions(): returns a vector of arbitrary size of strings containing all of the prior bluetooth transmissions

Furthermore, the other 5/9 functions not mentioned above as special cases all have a "get_all_X()" sibling (e.g., "get_all_heart_rate())
which returns the entire vector, not just the most recently observed value.

Lastly, main.rs contains comprehensive tests/examples of how to interact with the API.

## Power Consumption
In the simulation, the IOT wearable begins with 1000 "units" of power. For every sensor, each second in which that sensor is active, 1 unit of power is consumed. The function "blackboard.get_power_level()" returns the number 0-1000 of remaining power.