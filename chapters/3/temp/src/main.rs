fn celcius_to_farenheit(celcius: f64) -> f64 {
    celcius * 1.8 + 32.0
}

fn farenheit_to_celcius(farenheit: f64) -> f64 {
    (farenheit - 32.0) / 1.8
}

fn main() {
    println!("{}°C is {}°F", 0.0, celcius_to_farenheit(0.0));
    println!("{}°F is {}°C", 32.0, farenheit_to_celcius(32.0));
}
