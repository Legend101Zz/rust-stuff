use std::thread;
use std::time duration 


fn simulated_expensive_calculation(intensity:u32) -> u32{
println!("calculating slowing...");
thread::sleep(Duration::from_secs(2));
intensity
}