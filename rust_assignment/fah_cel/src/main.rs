const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT_F
}

fn main() {
    let mut temperature_f: f64 = FREEZING_POINT_F; // Start with freezing point (32°F)
    
    // Convert the initial temperature to Celsius and print
    let temperature_c = fahrenheit_to_celsius(temperature_f);
    println!("{:.1}°F is {:.1}°C", temperature_f, temperature_c);
    
    // Use a loop to convert and print the next 5 integer temperatures
    for _i in 1..=5 {
        temperature_f += 1.0; // Increment temperature
        let temperature_c = fahrenheit_to_celsius(temperature_f);
        println!("{:.1}°F is {:.1}°C", temperature_f, temperature_c);
    }
}