fn main() {
    println!("Hello, world!");
    let user1 = User {
        email: String::from("someemail@email.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1
    };
    println!("User1 email: {}", user1.email);
    let mut user_mutable = User {
        email: String::from("someemail@email.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1
    };
    println!("user_mutable email: {}", user_mutable.email);
    user_mutable.email = String::from("this_emailfield_has_changed");
    println!("user_mutable email: {}", user_mutable.email);

    let user_by_function_call = make_user(String::from("make_user_function"), String::from("make_user_function_email"));
    println!("user_by_function_call.username: {}", user_by_function_call.username);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("this is using struct update"),
        ..user2
    };
    println!("creating a user via struct update syntax: {}", user3.email);
    // update moves the value so it can't be reused, the following wouldn't compile due to user3 moving the values from user2
    // println!("user2.username after using update: {}", user2.username);
    // the following does compile, as a new value was created so one didn't have to be moved
    println!("user2.email after using update: {}", user2.email);
    // the following also compiles as active and sign_in_count are types that implement the Cpoy trait (bool, int)
    println!("user2.active after using update: {}", user2.active);


    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 24, 42);
    let _origin = Point(0, 0, 0);

    println!("black.0 index {}", black.0);
    println!("black.1 index {}", black.1);
    println!("black.2 index {}", black.2);

    // unit-like structs
    struct AlwaysEqual;
    let _subject = AlwaysEqual;



    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // Adding Useful Functionality with Derived Traits
    println!("rect1: Rectangle is: {:?}", rect1);
    let rect2 = Rectangle1 {
        _width: 30,
        _height: 50
    };
    println!("this is rect2: Rectangle1 using pretty print: {:#?}", rect2);

    dbg!("I'm a debug macro!");
    dbg!(&rect1);

    // dbg! returns the ownership of the expressions value:
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);

    // other derivable traits: https://doc.rust-lang.org/book/appendix-03-derivable-traits.html
    // attributes: https://doc.rust-lang.org/reference/attributes.html


    let rect_method = RectangleMethod {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect_method.area()
    );

    rect_method.area_as_string();

    if rect_method.width() {
        println!("The rectangle has a non zero width; it is {}", rect_method.width);
    }

    // methods with multiple params
    let rect_method_1 = RectangleMethod {
        width: 30,
        height: 50,
    };
    let rect_method_2 = RectangleMethod {
        width: 10,
        height: 40,
    };
    let rect_method_3 = RectangleMethod {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect_method_1.can_hold(&rect_method_2));
    println!("Can rect1 hold rect3? {}", rect_method_1.can_hold(&rect_method_3));


    // Associated functions
    let sq: RectangleMethod = RectangleMethod::square(5);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn make_user(username: String, email: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
// this makes Rectangle implement Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
#[derive(Debug)]
struct Rectangle1 {
    _width: u32,
    _height: u32
}
/*
    using '&' in the function signature means main retains its ownership and can continue using rect1 (what is passed into this function).
*/
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// methods
use std::fmt::Debug;
#[derive(Debug)]
struct RectangleMethod {
    width: u32,
    height: u32,
}

impl RectangleMethod {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn area_as_string(&self) {
        println!(
            "The area of the rectangle is {} square pixels.",
            self.width * self.height
        )
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, rect_to_compare: &RectangleMethod) -> bool {
        self.width > rect_to_compare.width && self.height > rect_to_compare.height 
    }
    fn square(size: u32) -> RectangleMethod {
        RectangleMethod {
            width: size,
            height: size
        }
    }
}