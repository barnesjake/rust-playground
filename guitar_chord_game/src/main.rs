use std::{cmp::Ordering, io};

use rand::Rng;

/*
    This program randomly prints a guitar chord to play.

    To add more chords:
    - just enter an asci value for it in the methods
    - increase the random number gen ran 
    - update match case to include new asci character
*/

fn main() {
    println!("Hello, welcome to chord printer!");
    /* 
        Modify this value to increase the time between chords being printed
        e.g. 2.5 for 2.5 seconds, 4.0 for 4 seconds.
    */
    const SLEEP_INTERVAL_IN_SECONDS: f32 = 2.0;

    loop {
        sleep_for_x_seconds(SLEEP_INTERVAL_IN_SECONDS);
        let random_number: i32 = rand::thread_rng().gen_range(1..5);
        match random_number {
            1 => print_a(),
            2 => print_c(),
            3 => print_d(),
            _ => print_e()
        }
    }

}

fn sleep_for_x_seconds(second_interval: f32) {
    use std::{thread, time};
    let sleep_delay = time::Duration::from_secs_f32(second_interval);
    thread::sleep(sleep_delay);
}

fn print_a() {
    println!("

       /\\ 
      /  \\
     / /\\ \\ 
    / ____ \\ 
   /_/    \\_\\ 
    ");
}
fn print_c() {
    println!("
     _____ 
    / ____|
   | |     
   | |     
   | |____ 
    \\_____|
    ");
}

fn print_e() {
    println!("
     ______ 
    |  ____|
    | |__   
    |  __|  
    | |____ 
    |______|
    ");
}


fn print_d() {
    println!("
     _____  
    |  __ \\ 
    | |  | |
    | |  | |
    | |__| |
    |_____/ 
    ");
}
