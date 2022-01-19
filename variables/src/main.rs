fn main() {
    
    // immutable variable
    // let x = 5;
    // mutable variable, denoted by the 'mut'
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // rust convention for const is all upercase naming
    // const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;


    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    // shadowing means we don't need to keep coming up with names... 
    // hardly seems worth it though imo...
    // but you can avoid compilation errors for quick things and change the type from string to a number
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces: {}", spaces);

    // this would have a compile error
    // let mut spaces = "   ";
    // spaces = spaces.len();

    println!("this is a function with a return type!: {}", five());
    println!("This is the plus_one call: {}", plus_one(five()));
    
    // control flow
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // loop {
        // println!("I am the loop")
    // }

    loop_with_breaks_example();
    return_value_from_loop_example();
    while_loop_example();
    for_loop_through_collection_example();
    more_concise_for_loop_through_collection_example();
    more_concise_for_loop_lift_off_example();
}

// functions
// -> denotes a return type
fn five() -> u32 {
    5
}

fn plus_one(x: u32) -> u32 {
    x + 1
}

fn loop_with_breaks_example() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn return_value_from_loop_example() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loop_example() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop_through_collection_example() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn more_concise_for_loop_through_collection_example() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

fn more_concise_for_loop_lift_off_example() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}