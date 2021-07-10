use std::io;

fn main() {
    println!("\
        [INFO] Starting Fahrenheit to Celsius \
        converter...");

    println!("\
        [INFO] Waiting for input...");

    let mut fahrenheit = String::new();

    io::stdin().read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: i32 =
        fahrenheit
            .trim()
            .parse()
            .expect("");

    let celsius: i32 = (fahrenheit - 32) * 5 / 9;

    println!(
        "[INFO] ({}°F − 32) × 5/9 = {}°C",
        fahrenheit, celsius);
}
