// The Fibonacci numbers are the numbers in the following integer sequence.
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, ...
// Given a number n, print n-th Fibonacci Number. 

use std::{io};

fn main() {
    let number_to_fib_on = take_number_input();
    println!("N-th number input: {}", number_to_fib_on);
    println!("N-th Fibonacci Number: {}", calculate_fibonacci(number_to_fib_on));
}

fn take_number_input() -> u32 {
    let number_to_return: u32;
    loop {
        println!("Please input your number.");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let valid_number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if valid_number <= 1 {
            number_to_return = 1;
            break;
        }
        else {
            number_to_return = valid_number;
            break;
        }
    }
    number_to_return
}

fn calculate_fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    }
    else {
        calculate_fibonacci(n-1) + calculate_fibonacci(n-2)
    }
}
