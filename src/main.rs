///Used ChatGPT to add Comments to every line for future reference - Self Coded otherwise

use std::io; // Import the standard input/output library

// Define a function to get the temperature in Fahrenheit from the user
fn f_entry() -> i32 {
    println!("Enter Temperature in Fahrenheit"); // Print a message to prompt the user for input
    let mut fahrenheit = String::new(); // Declare a mutable string variable to store user input
    
    // Read the user input from the standard input (stdin) and store it in the fahrenheit variable
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line"); // Handle any errors that occur during input
    
    // Parse the input string to an integer, trimming any leading or trailing whitespace
    let fahrenheit: i32 = fahrenheit.trim().parse().expect("invalid input"); 
    
    fahrenheit // Return the temperature in Fahrenheit
}

// Define a function to get the temperature in Celsius from the user
fn c_entry() -> i32 {
    println!("Enter Temperature in Celsius"); // Print a message to prompt the user for input
    let mut celsius = String::new(); // Declare a mutable string variable to store user input
    
    // Read the user input from the standard input (stdin) and store it in the celsius variable
    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line"); // Handle any errors that occur during input
    
    // Parse the input string to an integer, trimming any leading or trailing whitespace
    let celsius: i32 = celsius.trim().parse().expect("invalid input"); 
    
    celsius // Return the temperature in Celsius
}

fn main() {
    loop {
        let mut temp_type = String::new(); // Declare a mutable string variable to store user input
        
        println!("Are you starting with Celsius or Fahrenheit?"); // Print a message to prompt the user for input
        
        // Read the user input from the standard input (stdin) and store it in the temp_type variable
        io::stdin()
            .read_line(&mut temp_type)
            .expect("Failed to read line."); // Handle any errors that occur during input
        
        // Convert the input string to lowercase and remove leading or trailing whitespace
        let temp_answer: String = temp_type.trim().to_lowercase(); 
        
        // Check if the user input indicates starting with Fahrenheit
        if temp_answer == "f" {
            let fahrenheit = f_entry(); // Call the f_entry function to get the temperature in Fahrenheit
            
            // Calculate the temperature in Celsius using the Fahrenheit input
            let celsius = (fahrenheit - 32) * 5 / 9; 
            
            // Print the converted temperature
            println!("{} degrees Fahrenheit is {} degrees in Celsius.", fahrenheit, celsius);
        } 
        // Check if the user input indicates starting with Celsius
        else if temp_answer == "c" {
            let celsius = c_entry(); // Call the c_entry function to get the temperature in Celsius
            
            // Calculate the temperature in Fahrenheit using the Celsius input
            let fahrenheit = (celsius * 9 / 5) + 32;
            
            // Print the converted temperature
            println!("{} degrees Celsius is {} degrees in Fahrenheit.", celsius, fahrenheit);
        } 
        // If the user input is neither "f" nor "c", print an error message
        else { 
            println!("Invalid Entry");
        }
      
        let mut another = String::new(); // Declare a mutable string variable to store user input for continuing the loop
        println!("Convert another? (y/n)"); // Prompt the user to input whether to convert another temperature
        
        // Read the user input from the standard input (stdin) and store it in the another variable
        io::stdin()
            .read_line(&mut another)
            .expect("Failed to read line."); // Handle any errors that occur during input
        
        let another: String = another.trim().to_lowercase(); // Convert the input string to lowercase and remove leading or trailing whitespace
        
        // Break the loop if the user input is not "y"
        if another != "y" {
            break;
        }
    }
}
