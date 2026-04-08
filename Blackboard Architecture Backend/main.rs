use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use blackboard_arch::blackboard::{Blackboard, SIMULATE_REALTIME};
use blackboard_arch::sensor_loader::load_sensor_queue;

fn main() -> Result<(), Box<dyn Error>> {
    let mut queue = load_sensor_queue("sensor_data.json")?;
    let mut blackboard = Blackboard::new();

    let mut second_count = 0;
    while let Some(frame) = queue.pop_front() {
        second_count += 1;
        blackboard.apply_frame(frame.clone());

        // Clear water at second 30 (if not already cleared)
        if second_count == 30 {
            blackboard.clear_device_water();
            println!(">>> Water cleared at second 30 <<<");
        }

        //  Print all available data for this second
        /*
            Note: to remove the "Some"s that appear before the output, use this
            version of the code:
            blackboard.get_heart_rate().unwrap_or(0.0),
            blackboard.get_blood_pressure().unwrap_or_else(|| "N/A".to_string()),
            blackboard.get_barometric_pressure().unwrap_or(0.0),
            blackboard.get_time().unwrap_or(0.0),
            blackboard.get_temperature_outside().unwrap_or(0.0),
            blackboard.get_water_in_device(),
            blackboard.get_distance_traveled(),
            blackboard.get_steps_taken(),
            blackboard.get_bluetooth_transmissions()
         */
        println!(
            "Second {}:\n\
             Heart Rate: {:?} bpm\n\
             Blood Pressure: {:?}\n\
             Barometric Pressure: {:?} hPa\n\
             Time: {:?} s\n\
             Outside Temperature: {:?} °C\n\
             Water in Device: {}\n\
             Distance Traveled (total): {:.2} m\n\
             Steps Taken (total): {}\n\
             Bluetooth Transmissions: {:?}\n\
             Power Remaining: {} units\n",  // <-- ADDED power line
            second_count,
            blackboard.get_heart_rate(),
            blackboard.get_blood_pressure(),
            blackboard.get_barometric_pressure(),
            blackboard.get_time(),
            blackboard.get_temperature_outside(),
            blackboard.get_water_in_device(),
            blackboard.get_distance_traveled(),
            blackboard.get_steps_taken(),
            blackboard.get_bluetooth_transmissions(),
            blackboard.get_power_level()    // <-- ADDED power call
        );

        if SIMULATE_REALTIME {
            sleep(Duration::from_secs(1));
        }
    }

    // Final summary
    println!("\n--- Final Results ---");
    println!("Final Heart Rate: {:?}", blackboard.get_heart_rate());
    println!("Final Blood Pressure: {:?}", blackboard.get_blood_pressure());
    println!("Final Barometric Pressure: {:?}", blackboard.get_barometric_pressure());
    println!("Final Time: {:?}", blackboard.get_time());
    println!("Final Outside Temperature: {:?}", blackboard.get_temperature_outside());
    println!("Final Water in Device: {}", blackboard.get_water_in_device());
    println!("Total Distance Traveled: {:.2} m", blackboard.get_distance_traveled());
    println!("Total Steps Taken: {}", blackboard.get_steps_taken());
    println!("All Bluetooth Transmissions: {:?}", blackboard.get_bluetooth_transmissions());
    println!("Final Power Level: {} units", blackboard.get_power_level());  // <-- ADDED

    // Test the new "get_all_X" functions (print first 3 values or length)
    println!("\n--- Historical Data (get_all_*) ---");
    let all_hr = blackboard.get_all_heart_rate();
    println!("All heart rate values ({} total): first 3: {:?}", all_hr.len(), &all_hr[..all_hr.len().min(3)]);

    let all_bp = blackboard.get_all_blood_pressure();
    println!("All blood pressure values ({} total): first 3: {:?}", all_bp.len(), &all_bp[..all_bp.len().min(3)]);

    let all_baro = blackboard.get_all_barometric_pressure();
    println!("All barometric pressure values ({} total): first 3: {:?}", all_baro.len(), &all_baro[..all_baro.len().min(3)]);

    let all_time = blackboard.get_all_time();
    println!("All time values ({} total): first 3: {:?}", all_time.len(), &all_time[..all_time.len().min(3)]);

    let all_temp = blackboard.get_all_temperature_outside();
    println!("All outside temperature values ({} total): first 3: {:?}", all_temp.len(), &all_temp[..all_temp.len().min(3)]);

    Ok(())
}