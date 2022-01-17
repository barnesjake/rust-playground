
// Start with the temperature in Fahrenheit (e.g., 100 degrees).
// Subtract 32 from this figure (e.g., 100 - 32 = 68).
// Divide your answer by 1.8 (e.g., 68 / 1.8 = 37.78)

use std::{io};
use std::num::ParseFloatError;

fn main() {
    println!("Hello, enter a value to convert to celcius!");

    loop {

        let mut _temp_temperature_input = String::new();
        
        io::stdin()
        .read_line(&mut _temp_temperature_input)
        .expect("Failed to read line");
        
        let input: f32 = match parse_string_to_f32_decimal_result(_temp_temperature_input) {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You have entered {}", input);

        let result = fahrenheit_to_celcius(input);

        println!("{} Fahrenheit is {} Celcius", input, result);
        break;
    }

}

fn fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

fn parse_string_to_f32_decimal_result(str: String) -> Result<f32, ParseFloatError> {
    str.trim().parse()
}
