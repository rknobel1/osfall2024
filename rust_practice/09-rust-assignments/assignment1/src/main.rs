// const FAHRENHEIT:f64 = 32.0;

fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// fn celcius_to_fahrenheit(c: f64) -> f64 {
//     c * 9.0 / 5.0 + 32.0
// }

fn main() {
    let mut temp: f64 = 32.0;
    let mut c_temp = fahrenheit_to_celcius(temp);
    println!("{} F: {} C", temp, c_temp);

    println!("Printing next 5");
    let mut counter = 1.0;
    temp += 1.0;
    while counter <= 5.0 {
        c_temp = fahrenheit_to_celcius(temp);
        println!("{} F: {} C", temp, c_temp);
        counter += 1.0;
        temp += 1.0;
    }
}