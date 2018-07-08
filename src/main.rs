use std::io;

fn main() {
    let mut temperature_for_conversion: String = String::new();
    let mut temperature_for_conversion_float: f32 = 0 as f32;
    let mut temperature_scale: String = String::new();
    let mut is_valid_temperature: bool = false;

    // Get a valid temperature.
    while !is_valid_temperature {
        println!("Please provide the temperature for conversion:");

        io::stdin()
            .read_line(&mut temperature_for_conversion)
            .expect("Failed to read line.");

        temperature_for_conversion_float = match temperature_for_conversion
            .trim()
            .parse() {
            Ok(num) => {
                is_valid_temperature = true;
                num
            }
            Err(_) => {
                println!("You must provide a valid temperature.");
                temperature_for_conversion = String::new();
                continue;
            }
        };
    }


    // Get a valid temperature_scale.
    loop {
        println!("Please, provide the target temperature scale (F/C):");

        io::stdin()
            .read_line(&mut temperature_scale)
            .expect("Failed to read line.");

        temperature_scale = temperature_scale.trim().to_string();

        if temperature_scale == "F" || temperature_scale == "C" {
            break;
        } else {
            temperature_scale = String::new();
            println!("You must provide 'F' for Fahrenheit or 'C' for Celsius, as the temperature scale.");
        }
    }


    // Make tha calculation based on the temperature_scale.
    if temperature_scale == "F" {
        println!("{} Celcius = {} Fahrenheit", temperature_for_conversion_float, (temperature_for_conversion_float * 1.8 + 32 as f32));
    } else {
        println!("{} Fahrenheit = {} Celcius", temperature_for_conversion_float, ((temperature_for_conversion_float - 32 as f32) * 0.5556));
    }
}
