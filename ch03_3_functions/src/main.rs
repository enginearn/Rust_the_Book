fn main() {
    println!("Hello, world!");

    another_function_1();
    another_function_2(5);
    print_labeled_measurement(5, 'h');

    let y = 6; // This is a statement

    // let x = (let y = 6); // got an error

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is {}", x);

}

fn another_function_1() {
    println!("Another function!");
}

fn another_function_2(x: i32) {
    println!("The value of x is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // x + 1; // rustc -suggested help: consider removing this semicolon.
    x + 1 // without a semicolon, it became it works correctly
}

// called outside of main() doesn't work
// another_function_2::(6);