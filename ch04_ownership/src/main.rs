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

    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);

    println!("Ref: The length of '{}' is {}", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{} from change!", &mut s);

    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    { // As always, we can use curly brackets `{}` to create a new scope,
      // allowing for multiple mutable references, just not simultaneous ones:
        let r1 = &mut s;

        println!("This is a inside value '{}' of r1!", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems

    let r2 = &mut s;

    println!("A value inside &mut s borrowed from r1 is '{}'!!", r2);

    let mut s = String::from("hello");

    let r1 = &s;     // no problem
    let r2 = &s;     // no problem
    println!("'{}' from r1 and '{}' from r2!!", r1, r2);
    // variable r1 and r2 will not be used after this point

    let r3 = &mut s; // BIG problem was gone

    println!("'{}' from r3!!!", r3);

    let reference_to_nothing = dangle();

    let mut s = String::from("hello world!");

    let word = first_word_1(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    println!("word has a meaningless value '{}'? \nThis code will be error prone...", word);

    let s = String::from("hello world!");

    let hello = &s[0..5];
    let world = &s[6..11];
    let ex = &s[11..12];

    println!("hello: '{}', world: '{}'", hello, world);
    println!("hello: '{}', world: '{}' and ex: '{}'", hello, world, ex);

    // word still has the value 5 here, but there's no more string that
    // we could meaningfull use the value 5 with. word is now totally invalid!

    let s = String::from("hello");
    let len = s.len();

    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    let slice3 = &s[3..len];
    let slice4 = &s[3..];
    let slice5 = &s[0..len];
    let slice6 = &s[..];
    
    println!("slice1 &s[0..2]: {}", slice1);
    println!("slice2 &s[..2]: {}", slice2);
    println!("slice3 &s[3..len]: {}", slice3);
    println!("slice4 &s[3..]: {}", slice4);
    println!("slice5 &s[0..len]: {}", slice5);
    println!("slice6 &s[..]: {}", slice6);

    let word1 = String::from("dangling pointer");
    let f_w_2 = first_word_2(&word1);
    println!("{}", f_w_2);

    let word1 = String::from("あいうえお かきくけこ");
    let s_w = first_word_2(&word1);
    println!("{}", s_w);

    let mut s = String::from("hello world");

    let word = first_word_2(&s);

    // s.clear(); // error!

    println!("the first word is: '{}'", word);

    let my_string = String::from("hello world");

    // `first_word_3` works on slices of `String`s, whether partial or whole.
    let word1 = first_word_3(&my_string[0..6]);
    let word2 = first_word_3(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word3 = first_word_3(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole.
    let word4 = first_word_3(&my_string_literal[0..6]);
    let word5 = first_word_3(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word6 = first_word_3(my_string_literal);

    println!("'{}'", word1);
    println!("'{}'", word2);
    println!("'{}'", word3);
    println!("'{}'", word4);
    println!("'{}'", word5);
    println!("'{}'", word6);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

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

fn calculate_length_ref(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope. But because it does not have ownership of
  // what it refers to, nothing happens.

fn change(some_string: &mut String) // <- change immutable to mutable
{
    some_string.push_str(", world!");
}

// Let's change reference s to s directly!
fn dangle() -> String { // dangle returns a reference to a string
    let s = String::from("hello"); // s is a new String

    //&s // we return a reference to the String, s
    s   // Let's change reference s to s directly!
} // s goes out of scope, and is dropped. Its memory goes away.
  // DANGER!! was gone!!!!

fn first_word_1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..];
        }
    }
    &s[..]
}

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn second_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.item().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..i]
// }