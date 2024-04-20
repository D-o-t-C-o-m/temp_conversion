///used ChatGPT to Optimize my code from Main.rs

use std::io;

/// Function to get the temperature from the user input
fn get_temperature(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt); // Print the prompt to ask for temperature
        let mut input = String::new(); // Declare a mutable string variable to store user input
        
        // Read the user input from the standard input (stdin) and store it in the input variable
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line."); // Handle any errors that occur during input
        
        // Parse the input string to an integer, trimming any leading or trailing whitespace
        match input.trim().parse() {
            Ok(temperature) => return temperature, // Return the temperature if parsing is successful
            Err(_) => println!("Invalid input. Please enter a valid temperature."), // Print error message for invalid input
        }
    }
}

fn main() {
    loop {
        // Get user input for temperature type (Celsius or Fahrenheit)
        let temp_type = get_temperature("Are you starting with Celsius or Fahrenheit? (c/f)");

        // Get the temperature value based on the user input
        let temperature = match temp_type.to_lowercase().as_str() {
            "c" => get_temperature("Enter Temperature in Celsius:"), // If starting with Celsius, prompt for Celsius temperature
            "f" => get_temperature("Enter Temperature in Fahrenheit:"), // If starting with Fahrenheit, prompt for Fahrenheit temperature
            _ => {
                println!("Invalid input. Please enter 'c' for Celsius or 'f' for Fahrenheit."); // Print error message for invalid input
                continue; // Restart the loop to get valid input
            }
        };

        // Convert Fahrenheit to Celsius or Celsius to Fahrenheit based on user input
        let (from_unit, to_unit, converted_temp) = if temp_type.to_lowercase() == "c" {
            ("Celsius", "Fahrenheit", (temperature * 9 / 5) + 32) // Convert Celsius to Fahrenheit
        } else {
            ("Fahrenheit", "Celsius", (temperature - 32) * 5 / 9) // Convert Fahrenheit to Celsius
        };

        // Print the converted temperature
        println!("{} degrees {} is {} degrees in {}.", temperature, from_unit, converted_temp, to_unit);

        // Ask user if they want to convert another temperature
        println!("Convert another? (y/n)");
        let mut another = String::new();
        
        // Read the user input from the standard input (stdin) and store it in the another variable
        io::stdin()
            .read_line(&mut another)
            .expect("Failed to read line."); // Handle any errors that occur during input
        
        // Trim any leading or trailing whitespace and convert the input string to lowercase
        let another = another.trim().to_lowercase();

        // Break the loop if the user input is not "y"
        if another != "y" {
            break;
        }
    }
}
