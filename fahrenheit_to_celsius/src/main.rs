use std::io;

fn main() {
    let temperature = loop {
        println!("Enter c for Celsius and f for Fahrenehit.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");

        let input = input.trim();

        if input != "c" && input != "f" {
            continue;
        }

        println!("Enter the number.");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Error");

        let number: f32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == "c" {
            break convert_to_fahrenheit(number);
        } else {
            break convert_to_celsius(number);
        }
    };

    println!("The temperature is: {}", temperature);
}

fn convert_to_celsius(num: f32) -> String {
    format!("{}°C", (num - 32.0) / 1.8)
}

fn convert_to_fahrenheit(num: f32) -> String {
    format!("{}°F", num * (9.0 / 5.0) + 32.0)
}
