fn main() {
    let s = "hello";
    let S = String::from("hello");
    let mut m_S = String::from("hello");

    m_S.push_str(", world!");

    println!("from m_S: {}", m_S);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {} too!", s1, s2);

    let s = String::from("hello"); // - move occurs because `s` has type `String`, 
                                   // which does not implement the `Copy` trait
    // let x = s;                     //  value used here after move

    // println!("This x assigned s before taking ownership : {}", x);

    takes_ownership(s);

    // println!("This x assigned s after taking ownership : {}", x);

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();        // gives_ownership moves its return value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back,
                                       // which also moves its return value into s3

    println!("what returned value inside s1 is {}", s1);
    // println!("what returned value inside s2 is {}", s2); // value borrowed here after move
    println!("what returned value inside s3 is {}", s3);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

} // drop function works here! x goes out of space, then s, But because s's value was moved,
  // nothing special happens.
  // s3 goes out of scope and is dropped. s2 was moved, so nothing happens.
  // s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("some_string is '{}'", some_string);
} // some_string goes out of scope and `drop` function is called.
  //The backing memory is free!

  fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("ome_integer is '{}'", some_integer);
} // some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it.

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a string
    (s, length)
}