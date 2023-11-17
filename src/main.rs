use std::io;

fn celsius_to_farenheit(x: f64) {
    let result = x * 9.0 / 5.0 + 32.0;
    println!("Farenheit: {result}");
}

fn farenheit_to_celsius(x: f64) {
    let result = (x - 32.0) * 5.0 / 9.0;
    println!("Celsius: {result}");
}

fn main() {
    loop {
        let mut option = String::new();
        let mut value = String::new();
        println!("This a temperature converter");
        println!("What would you like to do?");
        println!("1. Convert from celsius to farenheit.");
        println!("2. Convert from farenheit to celsius.");
        println!("3. Exit.");

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read the line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => {
                if num > 0 && num < 4 {
                    num
                } else {
                    println!("Invalid Option {num}");
                    continue;
                }
            }
            Err(_) => {
                println!("Invalid option");
                continue;
            }
        };

        if option == 3 {
            println!("See you soon!!!");
            break;
        }

        println!("Digit a value: ");
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read value");

        let value: f64 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option");
                continue;
            }
        };

        if option == 1 {
            celsius_to_farenheit(value);
        } else if option == 2 {
            farenheit_to_celsius(value);
        }

        println!("\n");
    }
}
