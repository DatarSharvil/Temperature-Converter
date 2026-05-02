use std::io;
fn celsius_to_farenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}
fn farenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}
fn main() {
    println!("Enter '1' for Celsius to Farenheit and '2' for vice-versa.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: cannot read line!");
    let choice: i32 = input.trim().parse().expect("Please enter valid numbers!");
    input.clear();
    match choice {
        1 => {
            println!("Enter temperature in Celsius: ");
            io::stdin().read_line(&mut input).expect("Unable to read line!");
            let c: f64 = input.trim().parse().expect("Enter valid numbers!");
            let f: f64 = celsius_to_farenheit(c);
            println!("Temperature in Farenheit: {}", f);
        }
        2 => {
            println!("Enter temperature in farenheit: ");
            io::stdin().read_line(&mut input).expect("Unable to read line!");
            let f: f64 = input.trim().parse().expect("Enter valid numbers!");
            let c: f64 = farenheit_to_celsius(f);
            println!("Temperature in Celsius: {}", c);
        }
        _ => println!("Invalid Choice! "),
    }
}