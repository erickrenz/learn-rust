use std::io;

fn main() {
    println!("Temperature Converter");
    println!("Please select your conversion type:");
    println!(" 1. Celsius to Fahrenheit");
    println!(" 2. Fahrenheit to Celsius");

    loop {
        let mut conversion_type = String::new();

        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line");

        // check user input and verify it is either 1 or 2
        let conversion_type: u32 = match conversion_type.trim().parse() {
            Ok(num) => {
                if num == 1 || num == 2 {
                    num
                } else {
                    println!("Please enter either 1 or 2");
                    continue;
                }
            }
            Err(_) => continue,
        };

        println!("");
        println!("Please enter the temperature you would like to convert:");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        // convert user input to i32
        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // perform conversion
        if conversion_type == 1 {
            println!(
                "{}°C is {}°F",
                temperature,
                celsius_to_fahrenheit(temperature)
            );
            break;
        } else {
            println!(
                "{}°F is {}°C",
                temperature,
                fahrenheit_to_celsius(temperature)
            );
            break;
        }
    }
    println!("");
}

// function that converts input from Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    let fahrenheit = (celsius * 9 / 5) + 32;
    fahrenheit
}

// function that converts input from Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    let celsius = (fahrenheit - 32) * 5 / 9;
    celsius
}
