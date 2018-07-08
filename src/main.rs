use std::io;

fn main() {
    let mut temperature_for_conversion: String = String::new();
    let mut scale: String = String::new();

    // Todo: Get a valid temperature.

    // Get a valid scale.
    loop {
        println!("Please provide the scale (F/C):");

        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line.");

        scale = scale.trim().to_string();

        if scale == "F" || scale == "C" {
            break;
        } else {
            println!("You must provide 'F' or 'C' as the scale.");
        }
    }


    // Make tha calculation based on the scale.
    if scale == "F" {
        print!("It is Fahreneight.")
    } else {
        print!("It is Celcius.")
    }
}
