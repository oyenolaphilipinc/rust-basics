use std::io;

fn main() {
    println!("Temperature Converter");

    loop {
        println!("Choose an option:");
        println!("1. Convert Fahrenheit to Celsius");
        println!("2. Convert Celsius to Fahrenheit");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter temperature in Fahrenheit:");
                let mut fahrenheit = String::new();
                io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
                let fahrenheit: f64 = match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("Equivalent temperature in Celsius: {:.2}°C", celsius);
            }
            2 => {
                println!("Enter temperature in Celsius:");
                let mut celsius = String::new();
                io::stdin().read_line(&mut celsius).expect("Failed to read line");
                let celsius: f64 = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("Equivalent temperature in Fahrenheit: {:.2}°F", fahrenheit);
            }
            3 => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a valid option (1-3).");
            }
        }
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
