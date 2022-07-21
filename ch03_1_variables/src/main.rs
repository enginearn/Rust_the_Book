use std::io;

fn main() {
    // mutable variable
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
    
    // scope
    let a = 5;
    
    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in the linner scope is: {}", a);
    }

    println!("The value of x is: {}", x);
    
    // types
    let spaces = "     ";
    let spaces = spaces.len();

    // let mut spaces = "     ";
    // spaces = spaces.len();

    // floating-point
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // numeric operation
    let sum = 5 + 10;

    println!("Addition: {}", sum);

    let difference = 95.5 - 4.3;

    println!("Subtraction: {}", difference);

    let product = 4 * 30;

    println!("Multiplication: {}", product);

    let quotient = 56.7 /32.2;
    let floored = 2 /3;

    println!("Division quotient: {}", quotient);
    println!("Division floored: {}", floored);

    let remainder = 43 % 5;

    println!("Remainder: {}", remainder);
    
    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{}, {}, {}", c, z, heart_eyed_cat);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // println!(tup);

    let tuple = (500, 6.4, 1);

    let (x, y, z) = tuple;

    println!("The value of y is: {}, z is: {}, x is {}", y, z, x);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // index 0 element of tuple

    let six_point_four = x.1; // index 1 element of tuple

    let one = x.2;           // index 2 element of tuple

    println!("{}, {}, {}", &five_hundred, &six_point_four, &one);

    // array
    let a = [1, 2, 3, 4, 5];

    for e in a {
        println!("{}", e);
    }

    let months = ["January", "February", "March", "April", "May", "June",
                  "July", "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    for i in a {
        println!("{}", i);
    }

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("{}, {}", first, second);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line...");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!!");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
