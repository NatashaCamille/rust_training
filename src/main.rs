use std::io;

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
  (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
  let mut fahrenheit_input = String::new();

  println!("Enter a temperature in Fahrenheit:");
  io::stdin().read_line(&mut fahrenheit_input).expect("Failed to read input");

  let fahrenheit = fahrenheit_input.trim().parse::<f64>().expect("Invalid input");

  let celsius = fahrenheit_to_celsius(fahrenheit);

  println!("Temperature in Fahrenheit: {:.2}°F", fahrenheit);
  println!("Temperature in Celsius: {:.2}°C", celsius);
}
