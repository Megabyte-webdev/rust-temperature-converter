use std::io;

fn main() {
    println!("Temperature Converter (Celcius to Farenheit)");

    let mut input = String::new();
    println!("Enter temperature in Celcius: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let celcius: f64 = input.trim().parse().expect(
        "Please enter a valid
number",
    );
    let farenheit = (celcius * 9.0 / 5.0) + 32.0;

    println!("Temperature in Farenheit: {:.2}", farenheit);
}
