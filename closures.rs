use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowing...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

//  ========= APPROACH WITHOUT USE OF CLOSURES =========
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println! { "Calculating slowing..." };
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups", expensive_closure(intensity));
        println!("Next do , {} situps !", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today , remember to stay hydrated");
        } else {
            println!("Today, run for {} minutes", expensive_closure(intensity));
        }
    }
}
