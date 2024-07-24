fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    let fahrenheit = 98.6;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{:.2} Fahrenheit is {:.2} Celsius", fahrenheit, celsius);
}
