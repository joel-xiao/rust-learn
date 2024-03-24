use std::io;

fn main() {
   // u32
    let guess: u32 = "42".parse().expect("Not a number!");

    // f64
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // addition
    let sum = 5 + 10;

    // subtraction

    let difference = 95.5 - 4.3;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为-1

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // char
    let c = 'z';
    let z:char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    // or
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    // 
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    // 
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    
    // stack
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // see the value of the element at index 10
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    
    let mut index = String::new();

    io::stdin().read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index must be a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
    


              




}
