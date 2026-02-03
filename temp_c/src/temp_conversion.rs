



const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0) 

}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0/5.0)) + (32 as f64) 


}

fn main() {
    
    let mut ftemp: f64 = 32.0;

    let mut starter: f64 = ftemp;

    ftemp = fahrenheit_to_celsius(ftemp);

    println!("{}°F is {}°C", starter, ftemp);

    ftemp = starter;

   for _ in 0..5 {
    ftemp += 1.0;
    starter = ftemp;
    ftemp = fahrenheit_to_celsius(ftemp);
    println!("{}°F is {}°C", starter, ftemp);
    ftemp = starter;
    }

}
