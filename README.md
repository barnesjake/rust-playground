# rust-playground
Repo for learning Rust

To compile: `rustc main.rs`

Build using cargo:

in the directory, run `cargo build`

this creates the runnable file, which can be run using `./target/debug/(filecreated)`

`cargo run` can be used to build and run the program in one command

to check the code compiles, run `cargo check`

`cargo build --release` creates a file in /releases and includes optimizations to make the rust code run faster



### Scalar types in rust:
integers, floating-point numbers, booleans and characters

#### Integers
integer has multiple lengths, e.g. 8bit, 16bit, 32bit, 64bit, 128bit and arch
they come in two types of signed and unsigned, unsigned means it will only ever be positive.
e.g.:
So an `i8` can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a `u8` can store numbers from 0 to 28 - 1, which equals 0 to 255.

Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.

Number literals	Example
Decimal	`98_222`
Hex	`0xff`
Octal	`0o77`
Binary	`0b1111_0000`
Byte (u8 only)	`b'A'`

#### Floating point
`f32` or `f64`, `f64` is default if not stated

#### Boolean
represented by `bool`

#### Characters
`let c = 'z';`
`let heart_eyed_cat = '😻';`

#### Compounds types
tuples and arrays

##### Tuples
once declared, have fixed length
`let tup: (i32, f64, u8) = (500, 6.4, 1);`

Access values using pattern matching
```
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {}", y);z
```

Or use a dot operator with the index of the value
```
let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
```

##### Arrays
Every element of an array must have the same type, they also have a fixed length once declared.
Vectors can change size and is similar to array.
To write the type, use square brackets:
`let a: [i32; 5] = [1, 2, 3, 4, 5];`

Initialise all elements with the same value:
`let a = [3; 5];` is the same as `let a = [3, 3, 3, 3, 3];` but in a more concise way.

```
let a = [1, 2, 3, 4, 5];
let first = a[0]; // 1
```


### loops in rust:
there are three types of loops:
`loop`, `while` and `for`