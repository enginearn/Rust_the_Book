# Rust the Book

---

## Chapter 1

<details>
<summary>rustup --version</summary>

``` PowerShell
$ rustup --version                                                                                                                                                                                   in pwsh at 15:03:41 
rustup 1.24.3 (ce5817a94 2021-05-31)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.61.0 (fe5b13d68 2022-05-18)`
```

</details>

<details>
<summary>rustc --version</summary>

``` PowerShell
$ rustc --version                                                                                                                                                                                    in pwsh at 15:06:14 
rustc 1.61.0 (fe5b13d68 2022-05-18)
```

</details>

<details>
<summary>cargo --version</summary>

``` PowerShell
$ cargo --version                                                                                                                                                                                    in pwsh at 15:07:40 
cargo 1.61.0 (a028ae42f 2022-04-29)
```

</details>

### Hello World

<details>
<summary>main.rs</summary>

``` main.rs
fn main() {
    println!("Hello, World!");
}
```

</details>

<details>
<summary>compile</summary>

``` PowerShell
$ rustc .\main.rs
main.exe
main.pdb
main.rs
```

</details>

### Hello Cargo

<details>
<summary>cargo new hello_cargo</summary>

``` PowerShell
$ cargo new hello_cargo                                                                                                                                                                       in pwsh at 16:07:31
Created binary (application) `hello_cargo` package

$ cd hello_cargo
```

</details>

<details>
<summary>cargo build</summary>

``` PowerShell
$ cargo build                                                                                                                                                                                 in pwsh at 16:24:04 
Compiling hello_cargo v0.1.0 (C:\Users\path\Development\Rust\projects\hello_cargo)
Finished dev [unoptimized + debuginfo] target(s) in 1.33s
```

</details>

<details>
<summary>execute</summary>

``` PowerShell
$ .\target\debug\hello_cargo.exe                                                                                                                                                              in pwsh at 16:24:16
Hello, world!
```

</details>

<details>
<summary>cargo run</summary>

``` PowerShell
$ cargo run                                                                                                                                                                                    in pwsh at 16:26:33 
Finished dev [unoptimized + debuginfo] target(s) in 0.01s
Running `target\debug\hello_cargo.exe`
Hello, world!
```

</details>

<details>
<summary>cargo check</summary>

``` PowerShell
$ cargo check                                                                                                                                                                                 in pwsh at 16:44:44 
Checking hello_cargo v0.1.0 (C:\Users\path\Development\Rust\projects\hello_cargo)
Finished dev [unoptimized + debuginfo] target(s) in 0.39s
```

</details>

---

## Chapter 2

### programming a Guessing Game

<details>
<summary>setting up a new project</summary>

``` PowerShell
$ cargo new guessing_game
Created binary (application) `guessing_game` package
cd guessing_game
```

</details>

<details>
<summary>In Rust, variables are immutable by default</summary>

``` Rust
let apple = 5; // immutable
let mut apple = 5; // mutable
```

`mut` means mutable.
After the value is assigned to the variable, the variable name is prefixed with `mut`.
it looks like `&mut apple` or `&apple` and `&` means ***reference***.

</details>

<details>
<summary>cargo build</summary>

``` PowerShell
$ cargo build
Compiling guessing_game v0.1.0 (C:\Users\path\Development\Rust\projects\guessing_game)
Finished dev [unoptimized + debuginfo] target(s) in 1.21s
```

</details>

<details>
<summary>cargo run</summary>

``` PowerShell
$ cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.01s
Running `target\debug\guessing_game.exe`
Guess the number!
Please input your guess.
3
You guessed: 3
```

</details>

<details>
<summary>Add rand functionality</summary>

``` Cargo.toml
rand = "0.8.5"
```

</details>

<details>
<summary>cargo build</summary>

``` PowerShell
cargo build
    Updating crates.io index
  Downloaded rand_core v0.6.3
  Downloaded rand_chacha v0.3.1
  Downloaded rand v0.8.5
  Downloaded ppv-lite86 v0.2.16
  Downloaded getrandom v0.2.6
  Downloaded cfg-if v1.0.0
  Downloaded 6 crates (182.5 KB) in 1.58s
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling getrandom v0.2.6
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (C:\Users\path\Development\Rust\projects\guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 7m 44s
```

</details>

<details>
<summary>cargo doc --open</summary>

``` PowerShell
cargo doc --open
    Checking cfg-if v1.0.0
    Checking ppv-lite86 v0.2.16
 Documenting cfg-if v1.0.0
 Documenting ppv-lite86 v0.2.16
    Checking getrandom v0.2.6
    Checking rand_core v0.6.3
 Documenting getrandom v0.2.6
    Checking rand_chacha v0.3.1
    Checking rand v0.8.5
 Documenting rand_core v0.6.3
 Documenting rand_chacha v0.3.1
 Documenting rand v0.8.5
 Documenting guessing_game v0.1.0 (C:\Users\path\Development\Rust\projects\guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 10.35s
     Opening C:\Users\path\Development\Rust\projects\guessing_game\target\doc\guessing_game\index.html
```

</details>

<details>
<summary>match guess.cmp(&secret_number)</summary>

`cmp` means ***compare***.

> A `match` expression is made up of arms. An arm consists of a pattern to match against,
> and the code that should be run if the value given to `match` fits that arm's pattern.

</details>

<details>
<summary>underscore `_` is a catch all values</summary>

``` rust
let guess: u32 =
        guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
```

``` rust
cargo run
   Compiling guessing_game v0.1.0 (C:\Users\path\Development\Rust\projects\guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.61s
     Running `target\debug\guessing_game.exe`
Guess the number!
The secret number is: 1
Please input your guess.
a
Please input your guess.
あ
Please input your guess.
1
You guessed: 1
You won!
```

</details>

---

## Chapter 3

### Common Programming Concepts

#### Variables and Mutability

<details>
<summary>cannot assign twice to immutable variable</summary>

``` rust
cargo run
   Compiling variables v0.1.0 (C:\Users\path\Development\Rust\projects\variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src\main.rs:5:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
...
5 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```

``` rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
}
```

``` rust
cargo run
   Compiling variables v0.1.0 (C:\Users\path\Development\Rust\projects\variables)
    Finished dev [unoptimized + debuginfo] target(s) in 1.62s
     Running `C:\Users\path\Development\Rust\projects\variables\target\debug\variables.exe`
The value of x is: 5
The value of x is: 6
```

</details>

#### Constants

<details>
<summary>const</summary>

``` rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

</details>

#### Shadowing

<details>
<summary>inner scope</summary>

``` rust
fn main() {
    let a = 5;

    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in the inner scope is: {}", a);
    }

    println!("The value of x is: {}", x);
}
```

``` rust
cargo run
   Compiling variables v0.1.0 (C:\Users\path\to\\Development\Rust\projects\variables)
warning: unused variable: `x`
  --> src\main.rs:13:13
   |
13 |         let x = x + 2;
   |             ^ help: if this is intentional, prefix it with an underscore: `_x`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `variables` (bin "variables") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 1.43s
     Running `C:\Users\path\to\\Development\Rust\projects\variables\target\debug\variables.exe
The value of x in the inner scope is: 5
The value of x is: 7
```

</details>

<details>
<summary>same variable names deferent data types</summary>

``` rust
let spaces = "     ";
    let spaces = spaces.len():

    let mut spaces = "     ";
    spaces = spaces.len();
```

``` rust
cargo run
   Compiling variables v0.1.0 (C:\Users\path\to\\Development\Rust\projects\variables)
error[E0308]: mismatched types
  --> src\main.rs:23:14
   |
22 |     let mut spaces = "     ";
   |                      ------- expected due to this value
23 |     spaces = spaces.len();
   |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```

</details>

#### Data Types

**Integer Types**

|**Length**|**Signed**|**Unsigned**|
| :---: | :--- | :--- |
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i28|u128|
|arch|isize|usize|

**Integer literals in Rust**

|**Number Literals**|**Example**|
|:---|---|
|Decimal|98_222|
|Hex|0xff|
|Octal|0o77|
|Binary|0b1111_0000|
|Bytes(`u8` only)|b'A'|

#### Floating-Point Types

<details>
<summary>Floating point</summary>

``` rust
let x = 2.0; // f64

let y: f32 = 3.0; // f32
```

</details>

#### Numeric Operations

<details>
<summary>calculation</summary>

``` rust
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
```

</details>

<details>
<summary>results</summary>

``` rust
Addition: 15
Subtraction: 91.2
Multiplication: 120
Division quotient: 1.7608695652173911
Division floored: 0
Remainder: 3
```

</details>

#### Boolean Type

<details>
<summary>true / false</summary>

``` rust
let t = true;
let f: bool = false; // with explicit type annotation
```

</details>

#### Character Type

> `char` type is 4 bytes.
> `char` literals with single quotes `'` as opposed to `string` literals with double quotes `"`

<details>
<summary>char</summary>

``` rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';

println!("{}, {}, {}", c, z, heart_eyed_cat);
```

</details>

<details>
<summary>results</summary>

``` rust
z, ℤ, 😻
```

</details>

#### Compound Types

<details>
<summary>tuple</summary>

``` rust
// tuple
let tup: (i32, f64, u8) = (500, 6.4, 1);

// println!(tup);

let tuple = (500, 6.4, 1);

let (x, y, z) = tuple;

println!("The value of y is: {}, z is: {}, x is {}", y, z, x);
```

</details>

<details>
<summary>results</summary>

``` rust
The value of y is: 6.4, z is: 1, x is 500
```

</details>

<details>
<summary>indexes in a tuple</summary>

``` rust
let x: (i32, f64, u8) = (500, 6.4, 1);

let five_hundred = x.0; // index 0 element of tuple

let six_point_four = x.1; // index 1 element of tuple

let one = x.2;           // index 2 element of tuple

println!("{}, {}, {}", &five_hundred, &six_point_four, &one);
```

</details>

<details>
<summary>results</summary>

``` rust
500, 6.4, 1
```

</details>

<details>
<summary>Array</summary>

``` rust
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
```

</details>

<details>
<summary>results</summary>

``` rust
The value of y is: 6.4, z is: 1, x is 500
500, 6.4, 1
1
2
3
4
5
3
3
3
3
3
1, 2
Please enter an array index.
3
The value of the element at index 3 is: 4
Please enter an array index.
7
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 7', src\main.rs:130:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `C:\Users\path\to\\Development\Rust\projects\variables\target\debug\variables.exe` (exit code: 101)
```

</details>

#### Parameters

<details>
<summary>call function</summary>

``` rust
fn main() {
    println!("Hello, world!");

    another_function_1();
    another_function_2(5);
}

fn another_function_1() {
    println!("Another function!");
}

fn another_function_2(x: i32) {
    println!("The value of x is {}", x);
}

// called outside of main() doesn't work
// another_function_2::(6);
```

</details>

<details>
<summary>results</summary>

``` rust
❯ cargo run
   Compiling functions v0.1.0 (C:\Users\path\to\\Development\Rust\projects\functions)
error: expected one of `!` or `::`, found `(`
  --> src\main.rs:16:19
   |
16 | another_function_2(6);
   |                   ^ expected one of `!` or `::`

error: could not compile `functions` due to previous error
```

</details>

#### Statements and Expressions

<details>
<summary>Statement</summary>

``` rust
let y = 6; // This is a statement

    let x = (let y = 6); // got an error
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling functions v0.1.0 (C:\Users\path\to\Development\Rust\projects\functions)
error: expected expression, found statement (`let`)
  --> src\main.rs:10:14
   |
10 |     let x = (let y = 6);
   |              ^^^^^^^^^
   |
   = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
  --> src\main.rs:10:14
   |
10 |     let x = (let y = 6);
   |              ^^^^^^^^^
   |
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

warning: unnecessary parentheses around assigned value
  --> src\main.rs:10:13
   |
10 |     let x = (let y = 6);
   |             ^         ^
   |
   = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
   |
10 -     let x = (let y = 6);
10 +     let x = let y = 6;
   | 

For more information about this error, try `rustc --explain E0658`.
warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` due to 2 previous errors; 1 warning emitted
```

</details>

Other languages such as C lang, Ruby, you can write `x = y = 6` and the result is `x` and `y` assigned 6.
it does'nt work in Rust.

<details>
<summary>Expression</summary>

``` rust
let y = {
        let x = 3;
        x + 1 // it does'nt have a semicolon
    };
```

> **Warning**
> Expressions do not include ending semicolons. If you add a semicolon to the end of an expression,
> you turn it into a statement, and it will then not return a value.
> Keep in mind as you explore ***function return values and expressions next.***

</details>

#### Functions with Return Values

<details>
<summary>Return value</summary>

``` rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling functions v0.1.0 (C:\Users\path\to\Development\Rust\projects\functions)
   Finished dev [unoptimized + debuginfo] target(s) in 1.43s
     Running `C:\Users\path\to\Development\Rust\projects\functions\target\debug\functions.exe`
The value of x is: 5
```

</details>

<details>
<summary>another example</summary>

``` rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is {}", x);
}

fn plus_one(x: i32) -> i32 {
    // x + 1; // rustc -suggested help: consider removing this semicolon.
    x + 1 // without a semicolon, it became it works correctly
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling functions v0.1.0 (C:\Users\path\to\Development\Rust\projects\functions)
   Finished dev [unoptimized + debuginfo] target(s) in 1.22s
     Running `C:\Users\path\to\Development\Rust\projects\functions\target\debug\functions.exe`
The value of x is 6
```

</details>

#### Comments

~~There is nothing special.~~

> Warning
> DO NOT LEAVE COMMENTS STRANGE AREA!

<details>
<summary>got an error!</summary>

``` rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{} from change!", &mut s);
}

fn change(some_string: &mut String // <- change immutable to mutable) {
    some_string.push_str(", world!");
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
error: expected one of `!`, `(`, `)`, `,`, `::`, or `<`, found `some_string`
   --> src\main.rs:109:10
    |
109 | fn change(some_string: &mut String // <- change immutable to mutable) {
    |          ^ unclosed delimiter     - help: `)` may belong here
110 |     some_string.push_str(", world!");
    |     ^^^^^^^^^^^

error: expected one of `->`, `where`, or `{`, found `some_string`
   --> src\main.rs:110:5
    |
109 | fn change(some_string: &mut String // <- change immutable to mutable) {
    |                                   - expected one of `->`, `where`, or `{`
110 |     some_string.push_str(", world!");
    |     ^^^^^^^^^^^ unexpected token

error: could not compile `ownership` due to 2 previous errors
```

</details>

<details>
<summary>got an error!</summary>

``` rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{} from change!", &mut s);
}

fn change(some_string: &mut String)// <- change immutable to mutable {
    some_string.push_str(", world!");
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
error: unexpected closing delimiter: `}`
   --> src\main.rs:111:1
    |
104 | fn calculate_length_ref(s: &String) -> usize { // s is a reference to a String
    |                                              - this opening brace...
105 |     s.len()
106 | } // s goes out of scope. But because it does not have ownership of
    | - ...matches this closing brace
...
111 | }
    | ^ unexpected closing delimiter

error: could not compile `ownership` due to previous error
```

</details>

<details>
<summary>this works correctly!</summary>

``` rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{} from change!", &mut s);
}

fn change(some_string: &mut String) { // <- change immutable to mutable
    some_string.push_str(", world!");
}
```

</details>

<details>
<summary>results</summary>

``` rust
hello, world! from change!
```

</details>

If you want to leave comments after the brace, you need to beginning an a new line.

<details>
<summary>this works correctly too!</summary>

``` rust
fn change(some_string: &mut String) // <- change immutable to mutable
{
    some_string.push_str(", world!");
}
```

</details>

<br>

I use `Python` mainly so I'm not familiar with parentheses `()`, curly braces `{}`, and semicolons`;` yet
especially the start writing codes of functions...

### Control Flow

#### if Expressions

<details>
<summary>example</summary>

``` rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }

}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling control_flow v0.1.0 (C:\Users\path\to\Development\Rust\projects\control_flow)
    Finished dev [unoptimized + debuginfo] target(s) in 1.32s
     Running `C:\Users\path\to\Development\Rust\projects\control_flow\target\debug\control_flow.exe`
condition was true
```

</details>

<details>
<summary>evaluate equal is not true?</summary>

> the condition in this code below must be a bool.
> if condition isn't not bool,we'll got an error.

It looks to be true...

``` rust
let number = 3;
if number {
        println!("The number was three");
    }
```

</details>

**Warning**
> ***Rust will not automatically try to convert non-Boolean types to a Boolean.***
> ***You must explicit and always provide if with a Boolean as its condition.***


<details>
<summary>results</summary>

``` rust
cargo run
   Compiling control_flow v0.1.0 (C:\Users\path\to\Development\Rust\projects\control_flow)
error[E0308]: mismatched types
  --> src\main.rs:11:8
   |
11 |     if number {
   |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `control_flow` due to previous error
```

</details>

#### Using if in let statement

<details>
<summary>if in let</summary>

``` rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("TThe value of number is {}", number);
}
```

</details>

<details>
<summary>ersults</summary>

``` rust
gar in src on   dev
❯ cargo run
   Compiling control_flow v0.1.0 (C:\Users\path\to\Development\Rust\projects\control_flow)
    Finished dev [unoptimized + debuginfo] target(s) in 1.11s
     Running `C:\Users\path\to\Development\Rust\projects\control_flow\target\debug\control_flow.exe`
The value of number is 5
```

</details>

<details>
<summary>if must be the same type</summary>

``` rust
fn main() {
    let number = if condition { 5 } else [ "six" ];

    println!("The value of number is {}", number);
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling control_flow v0.1.0 (C:\Users\path\to\Development\Rust\projects\control_flow)
error[E0308]: `if` and `else` have incompatible types
  --> src\main.rs:38:44
   |
38 |     let number = if condition { 5 } else { "six" };
   |                                 -          ^^^^^ expected integer, found `&str`
   |                                 |
   |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `control_flow` due to previous error
```

</details>

#### Repetition with Loop

<details>
<summary>Repeating with loop</summary>

``` rust
fn main() {
    loop {
        println!("Hello, world!");
    }
}
```

</details>

<details>
<summary>results</summary>

``` rust
Hello, world!
Hello, world!
Hello, world!
Hello, world!
Hello, world!
Hello, world!
Hello, world!
Hello, world!
Hello, world!
Hello, world!
Hello, world!
Hello, world!
Hello, world!
Hello, world!
Hello, world!
```

</details>

<details>
<summary>nested loops</summary>

``` rust
fn main() {
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

```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling loops v0.1.0 (C:\Users\path\to\Development\Rust\projects\loops)
    Finished dev [unoptimized + debuginfo] target(s) in 1.20s
     Running `C:\Users\path\to\Development\Rust\projects\loops\target\debug\loops.exe`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

</details>

#### Returning Values from Loops

<details>
<summary>Return value</summary>

``` rust
let mut  counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result)
```

</details>

<details>
<summary>resukt</summary>

``` rust
cargo run
   Compiling loops v0.1.0 (C:\Users\path\to\Development\Rust\projects\loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.90s
     Running `C:\Users\path\to\Development\Rust\projects\loops\target\debug\loops.exe`
The result is 20
```

</details>

#### Conditional Loops with While

<details>
<summary>while</summary>

``` rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!");
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling loops v0.1.0 (C:\Users\path\to\Development\Rust\projects\loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.87s
     Running `C:\Users\path\to\Development\Rust\projects\loops\target\debug\loops.exe`
3!
2!
1!
LIFTOFF!
```

</details>

#### Looping Through a Collection with for

<details>
<summary>Loop with compound types</summary>

``` rust

```

</details>


<details>
<summary>results</summary>

``` rust
cargo run
   Compiling loops v0.1.0 (C:\Users\path\to\Development\Rust\projects\loops)
    Finished dev [unoptimized + debuginfo] target(s) in 1.64s
     Running `C:\Users\path\to\Development\Rust\projects\loops\target\debug\loops.exe`
The value is: 10
The value is: 20
The value is: 30
The value is: 40
The value is: 50
```

</details>

## Chapter 4

### Ownership

#### Ownership Rules

- Each value in Rust has a variable that's called owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

#### The String Type

<details>
<summary>String::from("hello");</summary>

> The double colon `::` operator allows us to namespace thsi particular from function
> under the `String` type rather than using some sort of name like `string_from`.


``` rust
fn main() {
    let s = "hello";
    let S = String::from("hello");
    let mut m_S = String::from("hello");

    m_S.push_str(", world!");

    println!("from m_S: {}", m_S);

    // drop function works around from here
}   // to here!
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
warning: unused variable: `s`
 --> src\main.rs:2:9
  |
2 |     let s = "hello";
  |         ^ help: if this is intentional, prefix it with an underscore: `_s`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `S`
 --> src\main.rs:3:9
  |
3 |     let S = String::from("hello");
  |         ^ help: if this is intentional, prefix it with an underscore: `_S`

warning: variable `S` should have a snake case name
 --> src\main.rs:3:9
  |
3 |     let S = String::from("hello");
  |         ^ help: convert the identifier to snake case (notice the capitalization): `s`
  |
  = note: `#[warn(non_snake_case)]` on by default

warning: variable `m_S` should have a snake case name
 --> src\main.rs:4:13
  |
4 |     let mut m_S = String::from("hello");
  |             ^^^ help: convert the identifier to snake case (notice the capitalization): `m_s`

warning: `ownership` (bin "ownership") generated 4 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 1.66s
     Running `C:\Users\path\to\Development\Rust\projects\ownership\target\debug\ownership.exe`
from m_S: hello, world!
```

</details>

#### Memory and Allocation

#### Ways Valiables and Data Interact: Move

<details>
<summary>got an error!</summary>

``` rust

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}

```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
warning: unused variable: `s2`
  --> src\main.rs:14:9
   |
14 |     let s2 = s1;
   |         ^^ help: if this is intentional, prefix it with an underscore: `_s2`

error[E0382]: borrow of moved value: `s1`
  --> src\main.rs:16:28
   |
13 |     let s1 = String::from("hello");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
14 |     let s2 = s1;
   |              -- value moved here
15 | 
16 |     println!("{}, world!", s1);
   |                            ^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
warning: `ownership` (bin "ownership") generated 4 warnings
error: could not compile `ownership` due to previous error; 4 warnings emitted
```

</details>

<details>
<summary>`clone` creates a deep copy</summary>

``` rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // `clone` creates a deep copy of s1

    println!("s1 = {}, s2 = {}", s1, s2);

    // drop function works around from here
}   // to here!
```

</details>

<details>
<summary>results</summary>

``` rust
 cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
   Finished dev [unoptimized + debuginfo] target(s) in 1.62s
     Running `C:\Users\path\to\Development\Rust\projects\ownership\target\debug\ownership.exe`
from m_S: hello, world!
s1 = hello, s2 = hello too!
```

</details>

#### Ownership and Functions

<details>
<summary>How Ownership works</summary>

``` rust
fn main() {

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

} // drop function works here! x goes out of space, then s, But because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("some_string is '{}'", some_string);
} // some_string goes out of scope and `drop` function is called.
  //The backing memory is free!

  fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("ome_integer is '{}'", some_integer);
} // some_integer goes out of scope. Nothing special happens.
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
   Finished dev [unoptimized + debuginfo] target(s) in 1.44s
     Running `C:\Users\path\to\Development\Rust\projects\ownership\target\debug\ownership.exe`
from m_S: hello, world!
x = 5, y = 5
s1 = hello, s2 = hello too!
some_string is 'hello'
ome_integer is '5'
```

</details>

<details>
<summary>Tries to use `s` after the call `takes_ownership`</summary>

``` rust
fn main() {

    let s = String::from("hello");
    let x = s;

    println!("This x assigned s before taking ownership : {}", x);
    takes_ownership(s);

    // println!("This x assigned s after taking ownership : {}", x);

    let x = 5;

    makes_copy(x);

} // drop function works here! x goes out of space, then s, But because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("some_string is '{}'", some_string);
} // some_string goes out of scope and `drop` function is called.
  //The backing memory is free!

  fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("ome_integer is '{}'", some_integer);
} // some_integer goes out of scope. Nothing special happens.

```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
error[E0382]: use of moved value: `s`
  --> src\main.rs:30:21
   |
25 |     let s = String::from("hello");
   |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
26 |     let x = s;
   |             - value moved here
...
30 |     takes_ownership(s);
   |                     ^ value used here after move
For more information about this error, try `rustc --explain E0382`.
warning: `ownership` (bin "ownership") generated 3 warnings
error: could not compile `ownership` due to previous error; 3 warnings emitted
```

</details>

#### Return Values and Scope

<details>
<summary>Return Values</summary>

``` rust
fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back,
                                       // which also moves its return value into s3

    println!("what returned value inside s1 is {}", s1);
    println!("what returned value inside s2 is {}", s2); // value borrowed here after move
    println!("what returned value inside s3 is {}", s3);
}

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

```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
error[E0382]: borrow of moved value: `s2`
  --> src\main.rs:47:53
   |
41 |     let s2 = String::from("hello");    // s2 comes into scope
   |         -- move occurs because `s2` has type `String`, which does not implement the `Copy` trait
42 | 
43 |     let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back,
   |                                   -- value moved here
...
47 |     println!("what returned value inside s2 is {}", s2);
   |                                                     ^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
warning: `ownership` (bin "ownership") generated 3 warnings
error: could not compile `ownership` due to previous error; 3 warnings emitted
```

</details>

<details>
<summary>return multiple values using a tuple</summary>

``` rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a string
    (s, length)
}
```

</details>

<details>
<summary>results</summary>

``` rust
The length of 'hello' is 5.
```

</details>

#### References and Borrowing

<details>
<summary>get return values using reference</summary>

``` rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);

    println!("Ref: The length of '{}' is {}", s1, len);
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}
```

</details>

<details>
<summary>results</summary>

``` rust
Ref: The length of 'hello' is 5
```

</details>

<details>
<summary>got an error!</summary>

``` rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
some_string.push_str(", world!");
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
--> src\main.rs:104:1
    |
103 | fn change(some_string: &String) {
    |                        ------- help: consider changing this to be a mutable reference: `&mut String`
104 | some_string.push_str(", world!");
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
warning: `ownership` (bin "ownership") generated 3 warnings
error: could not compile `ownership` due to previous error; 3 warnings emitted
```

</details>

#### Mutable References

<details>
<summary>got an error!</summary>

``` rust
fn main() {
    let s = String::from("hello");

    change(&s);

    println!("{} from change!", &s);
}

fn change(some_string: &String) {
some_string.push_str(", world!");
}

```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
   --> src\main.rs:108:1
    |
107 | fn change(some_string: &String) {
    |                        ------- help: consider changing this to be a mutable reference: `&mut String`
108 | some_string.push_str(", world!");
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
warning: `ownership` (bin "ownership") generated 3 warnings
```

</details>

<details>
<summary></summary>

``` rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{} from change!", &mut s);
}

fn change(some_string: &mut String) { // <- change immutable to mutable
    some_string.push_str(", world!");
}
```

</details>

<details>
<summary>results</summary>

``` rust
hello, world! from change!
```

</details>

<br>

> __Mutable references have one big restriction:__
> __you can have only one mutable reference to a particular piece of data at a time.__

<details>
<summary>got an error!</summary>

``` rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
  --> src\main.rs:70:14
   |
69 |     let r1 = &mut s;
   |              ------ first mutable borrow occurs here
70 |     let r2 = &mut s;
   |              ^^^^^^ second mutable borrow occurs here
71 | 
72 |     println!("{}, {}", r1, r2);
   |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
warning: `ownership` (bin "ownership") generated 3 warnings
error: could not compile `ownership` due to previous error; 3 warnings emitted
```

</details>

<br>

> A `data race` is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize access to the data.

> Data races cause undefined behavior and can be difficult to diagnose and fix
> when you're trying to track them down at runtime; Rust prevents this problem by
> refusing to compile cade with data races!

<br>

[データ競合(data race)と競合状態(race condition)を混同しない](https://qiita.com/yohhoy/items/00c6911aa045ef5729c6)

<details>
<summary>Borrowed a value</summary>

``` rust
fn main() {
    let mut s = String::from("hello");

    { // As always, we can use curly brackets `{}` to create a new scope,
      // allowing for multiple mutable references, just not simultaneous ones:
        let r1 = &mut s;

        println!("This is a inside value '{}' of r1!", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems

    let r2 = &mut s;

    println!("A value inside &mut s borrowed from r1 is '{}'!!", r2);
}
```

</details>

<details>
<summary>rsults</summary>

``` rust
This is a inside value 'hello' of r1!
A value inside &mut s borrowed from r1 is 'hello'!!
```

</details>

<br>

> Rust enforces a similar rule for combining mutable and immutable references.
> This code results in an error:

<details>
<summary>got an error!</summary>

``` rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;     // no problem
    let r2 = &s;     // no problem
    let r3 = &mut s; // BIG problem

    println!("{}, {}, and {}", r1, r2, r3);
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src\main.rs:89:14
   |
87 |     let r1 = &s;     // no problem
   |              -- immutable borrow occurs here
88 |     let r2 = &s;     // no problem
89 |     let r3 = &mut s; // BIG problem
   |              ^^^^^^ mutable borrow occurs here
90 |
91 |     println!("{}, {}, and {}", r1, r2, r3);
   |                                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
warning: `ownership` (bin "ownership") generated 3 warnings
error: could not compile `ownership` due to previous error; 3 warnings emitted
```

</details>

<br>

> Whew! We also cannot have a mutable reference while we have an immutable one to the same value.
> Users of an immutable reference don't expect the value to suddenly change out from under them!
> However, multiple immutable references are allowed because no one who is just reading the data
> has the ability to affect anyone else's reading the data.

<details>
<summary>this works correctly!</summary>

``` rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;     // no problem
    let r2 = &s;     // no problem
    println!("'{}' from r1 and '{}' from r2!!", r1, r2);
    // variable r1 and r2 will not be used after this point

    let r3 = &mut s; // BIG problem was gone

    println!("'{}' from r3!!!", r3);
}
```

</details>

<details>
<summary>results</summary>

``` rust
'hello' from r1 and 'hello' from r2!!
'hello' from r3!!!
```

</details>

<br>

[Announcing Rust 1.31 and Rust 2018](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes)

[Non-Lexical Lifetimes](https://qiita.com/_EnumHack/items/8b6ecdeb52e69a4ff384)

#### Dangling References

> A ___`dangling pointer`___ --a pointer that erferences a location in memory
> that may have been given to someone else--by freeing some memory while preserving a pointer to that memory.
> In Rust, by contract, the compiler guarantees that references will never be dangling references:
> if you have a reference to some data, the compiler will ensure that the data will not go out of scope
> before the reference to the data does.

<details>
<summary>got an error!</summary>

``` rust
fn main() {
    let reference_to_nothing = dangle();
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
error[E0106]: missing lifetime specifier
   --> src\main.rs:141:16
    |
141 | fn dangle() -> &String {
    |                ^ expected named lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
    |
141 | fn dangle() -> &'static String {
    |                ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` due to previous error
```

</details>

<details>
<summary>returning valiable String directly</summary>

``` rust
fn main() {
    let reference_to_nothing = dangle();
}

// Let's change reference s to s directly!
fn dangle() -> String { // dangle returns a reference to a string
    let s = String::from("hello"); // s is a new String

    //&s // we return a reference to the String, s
    s   // Let's change reference s to s directly!
} // s goes out of scope, and is dropped. Its memory goes away.
  // DANGER!! was gone!!!!
```

</details>

<br>

> **The Rules of References**
> Let's recap what we've discussed about references:
> - At any given time, you can have either one mutable reference or any number of immutable references.
> References must always be valid.

#### The Slice Type

> ***Slice*** let you reference a contiguous sequence of elements in a collection rather than the whole collection.
> A slice is a kind of reference, so it does not have ownership.

<details>
<summary>slices</summary>

``` rust
fn main() {
    let s = String::from("hello");
    let len = s.len();

    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    let slice3 = &s[3..len];
    let slice4 = &s[3..];
    let slice5 = &s[0..len];
    let slice6 = &s[..];
    
    println!("slice1 &s[0..2]: '{}'", slice1);
    println!("slice2 &s[..2]: '{}'", slice2);
    println!("slice3 &s[3..len]: '{}'", slice3);
    println!("slice4 &s[3..]: '{}'", slice4);
    println!("slice5 &s[0..len]: '{}'", slice5);
    println!("slice6 &s[..]: '{}'", slice6);

    let word2 = String::from("dangling pointer");
    let f_w_2 = first_word_2(&word2);
    println!("'{}'", f_w_2);
}
```

</details>

<details>
<summary>results</summary>

``` rust
slice1 &s[0..2]: he
slice2 &s[..2]: he
slice3 &s[3..len]: lo
slice4 &s[3..]: lo
slice5 &s[0..len]: hello
slice6 &s[..]: hello
dangling pointer
```

</details>

<details>
<summary>got an error!</summary>

``` rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word_2(&s);

    s.clear();

    println!("the first word is: '{}'", word);
}
```

</details>


<details>
<summary>results</summary>

``` rust
cargo run
   Compiling ownership v0.1.0 (C:\Users\path\to\Development\Rust\projects\ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
   --> src\main.rs:147:5
    |
145 |     let word = first_word_2(&s);
    |                             -- immutable borrow occurs here
146 | 
147 |     s.clear();
    |     ^^^^^^^^^ mutable borrow occurs here
148 | 
149 |     println!("the first word is: '{}'", word);
    |                                         ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
warning: `ownership` (bin "ownership") generated 5 warnings
error: could not compile `ownership` due to previous error; 5 warnings emitted
```

> Recall from the borrowing rules that if we have an immutable reference to something, we cannot
> also take a mutable reference. Because ***`clear`*** needs to truncate the ***`String`*** , it needs to get a
> mutable reference. The ***`println!`*** after the call to ***`clear`*** uses the reference in ***`word`*** , so the immutable reference must still be active at that point. Rust disallows the
> mutable reference in ***`clear`*** and the imuutable reference in ***`word`*** from existing
> at the same time, and compilation fails.
> Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!
> 

</details>

#### String Slices as Parameters

<details>
<summary></summary>

``` rust
fn main() {
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
```

</details>

<details>
<summary>results</summary>

``` rust
'helllo'
'helllo'
'helllo'
'hello_'
'hello_world'
'hello_world'
```

</details>

#### Other Slices

<details>
<summary>slice collections</summary>

``` rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
```

</details>

### Summary

> The Rust language gives you control over your memory usage in the same way as other systems programming languages,
> but having the owner of data automatically clean up that data when owner goes out of scope means
> you don't have to write and debug extra code to get this control.

## Chapter 5

### Using Structs to Structure Related Data

#### Defining and Instantiating Structs

<details>
<summary></summary>

``` rust
// out side of main() function
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_account: u64,
}
```

</details>

#### Creating an Instance and Access The Field Data

<details>
<summary></summary>

``` rust

```

</details>

<details>
<summary>results</summary>

``` rust

```

</details>

#### Creating Instances From Other Instances With Struct Update Syntax

<details>
<summary></summary>

``` rust

```

</details>

<details>
<summary>results</summary>

``` rust

```

</details>

#### Using Tuple Structs without Named Fields to Create Differen Types

<details>
<summary></summary>

``` 

```

</details>

#### Unit-Like Structs Without Any Fields

<details>
<summary>got an error!</summary>

``` rust
struct User_with_Error {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
};

fn main() {
    let user_3 = User_with_Error {
        email: "got-an-error!@error.com",
        username: "someusername404",
        active: true,
        sign_in_count: 1
    };
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling structs v0.1.0 (C:\Users\path\to\Development\Rust\projects\structs)
warning: type `User_with_Error` should have an upper camel case name
  --> src\main.rs:13:8
   |
13 | struct User_with_Error {
   |        ^^^^^^^^^^^^^^^ help: convert the identifier to upper camel case: `UserWithError`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

error[E0308]: mismatched types
  --> src\main.rs:54:16
   |
54 |         email: "got-an-error!@error.com",
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                |
   |                expected struct `String`, found `&str`

error[E0308]: mismatched types
  --> src\main.rs:55:19
   |
55 |         username: "someusername404",
   |                   ^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                   |
   |                   expected struct `String`, found `&str`

For more information about this error, try `rustc --explain E0308`.
warning: `structs` (bin "structs") generated 1 warning
error: could not compile `structs` due to 2 previous errors; 1 warning emitted
```

</details>

<details>
<summary>errors</summary>

``` rust
path\to in src on   dev
❯ cargo run
   Compiling structs v0.1.0 (C:\Users\path\to\Development\Rust\projects\structs)
error: expected item, found `;`
  --> src\main.rs:18:2
   |
18 | };
   |  ^ help: remove this semicolon
   |
   = help: braced struct declarations are not followed by a semicolon

warning: type `User_with_Error` should have an upper camel case name
  --> src\main.rs:13:8
   |
13 | struct User_with_Error {
   |        ^^^^^^^^^^^^^^^ help: convert the identifier to upper camel case: `UserWithError`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

error[E0308]: mismatched types
  --> src\main.rs:54:16
   |
54 |         email: "got-an-error!@error.com",
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                |
   |                expected struct `String`, found `&str`

error[E0308]: mismatched types
  --> src\main.rs:55:19
   |
55 |         username: "someusername404",
   |                   ^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                   |
   |                   expected struct `String`, found `&str`

For more information about this error, try `rustc --explain E0308`.
warning: `structs` (bin "structs") generated 1 warning
error: could not compile `structs` due to 3 previous errors; 1 warning emitted
path\to in src on   dev
❯ cargo run
   Compiling structs v0.1.0 (C:\Users\path\to\Development\Rust\projects\structs)
error[E0422]: cannot find struct, variant or union type `User_with_Error` in this scope
  --> src\main.rs:53:18
   |
13 | struct UserWithError {
   | -------------------- similarly named struct `UserWithError` defined here
...
53 |     let user_3 = User_with_Error {
   |                  ^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `UserWithError`

For more information about this error, try `rustc --explain E0422`.
error: could not compile `structs` due to previous error
path\to in src on   dev
❯ cargo run
   Compiling structs v0.1.0 (C:\Users\path\to\Development\Rust\projects\structs)
error[E0277]: `AlwaysEqual` doesn't implement `std::fmt::Display`
  --> src\main.rs:53:20
   |
53 |     println!("{}", subject);
   |                    ^^^^^^^ `AlwaysEqual` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `AlwaysEqual`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
  --> src\main.rs:56:16
   |
56 |         email: "got-an-error!@error.com",
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                |
   |                expected struct `String`, found `&str`

error[E0308]: mismatched types
  --> src\main.rs:57:19
   |
57 |         username: "someusername404",
   |                   ^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                   |
   |                   expected struct `String`, found `&str`

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `structs` due to 3 previous errors
path\to in src on   dev
❯ cargo run
   Compiling structs v0.1.0 (C:\Users\path\to\Development\Rust\projects\structs)
error[E0277]: `AlwaysEqual` doesn't implement `Debug`
  --> src\main.rs:53:22
   |
53 |     println!("{:?}", subject);
   |                      ^^^^^^^ `AlwaysEqual` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `AlwaysEqual`
   = note: add `#[derive(Debug)]` to `AlwaysEqual` or manually `impl Debug for AlwaysEqual`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
  --> src\main.rs:56:16
   |
56 |         email: "got-an-error!@error.com",
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                |
   |                expected struct `String`, found `&str`

error[E0308]: mismatched types
  --> src\main.rs:57:19
   |
57 |         username: "someusername404",
   |                   ^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
  --> src\main.rs:16:5
   |
16 |     email: String,
   |     ^^^^^^^^^^^^^

warning: field is never read: `sign_in_count`
  --> src\main.rs:17:5
   |
17 |     sign_in_count: u64,
   |     ^^^^^^^^^^^^^^^^^^

warning: function is never used: `build_user`
  --> src\main.rs:62:4
   |
62 | fn build_user(email: String, username: String) -> User {
   |    ^^^^^^^^^^

warning: `structs` (bin "structs") generated 9 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 2.25s
     Running `C:\Users\path\to\Development\Rust\projects\structs\target\debug\structs.exe`
user1 email is 'anotheremail@example.com'.
user2 name is 'someusername_123'
black.0 is '0'
orange.1 is '165'
path\to in src on   de
```

</details>

#### An Example Program Using Structs

<details>
<summary></summary>

``` rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}
```

</details>

<details>
<summary>results</summary>

``` rust
The area of the rectangle is 1500 square pixels.
```

</details>

<details>
<summary>refactoring with tuple</summary>

``` rust
fn main() {
    let rect1 = (40, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        // `ref` means refactoring
        ref_tuple_area(rect1)
    );
}

fn ref_tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

</details>

<details>
<summary>results</summary>

``` rust
The area of the rectangle is 2000 square pixels.
```

</details>

<details>
<summary>refectoring with structs: adding more meaning</summary>

``` rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        ref_struct_area(&rect1)
    );
}
```

</details>

<details>
<summary>results</summary>

``` rust
The area of the rectangle is 2500 square pixels.
```

</details>

#### Adding Useful Functionality with Derived Traits

<details>
<summary>got an error!</summary>

``` rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
            width: 60,
            height: 50
        };

    println!("rect1 is {}", rect1);
}
```

</details>

<details>
<summary>results</summary>

``` rust
cargo run
   Compiling structs v0.1.0 (C:\Users\path\to\Development\Rust\projects\structs)
error[E0277]: `Rectangle` doesn't implement `Debug`
  --> src\main.rs:98:31
   |
98 |     println!("rect1 is {:?}", rect1);
   |                               ^^^^^ `Rectangle` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `structs` due to previous error
```

</details>

<details>
<summary>put #[derive(Debug)] on Rectangle like below</summary>

``` rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){
    let rect1 = Rectangle {
        width: 60,
        height: 50
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1); // formatted output
}
```

</details>

<details>
<summary>results</summary>

``` rust
rect1 is Rectangle { width: 60, height: 50 }
rect1 is Rectangle {
    width: 60,
    height: 50,
}
```

</details>

<br>

> Another way to print a value using the ***`Debug`*** format is to use the ***`dbg! macro`*** ,
> which takes ownership of an expression, printsthe file and line number of where that ***`dbg!`*** macro
> calls occurs in your code along with the resolving value of that expression, and returns ownership of the value.

> **Note**
> Calling the ***`dbg!`*** macro prints to standard error console stream ( ***`stderr`*** ), 
> as opposed to ***`println!`*** which prints to the standard output console stream ( ***`stdout`*** ).

<details>
<summary></summary>

``` rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    dbg!(&rect1);
}
```

</details>

<details>
<summary>results</summary>

``` rust
rect1 is Rectangle {
    width: 60,
    height: 50,
}
[src\main.rs:104] 30 * scale = 60
[src\main.rs:108] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

</details>

#### Method Syntax

> ***Methods*** are similar to functions: we declare them with the ***`fn`*** keyword and a name,
> they can have parameters and, return value and they contain some code that's run when the method
> called from somewhere else.
> Unlike functions, methods are defined within the context of a struct (or an enum or a trait object),
> and their first parameter is always ***`self`*** , which represents the instance of themethod is being call on.

<details>
<summary></summary>

``` 

```

</details>


<details>
<summary></summary>

``` 

```

</details>

> **Note**
> we still need to use the ***`&`*** in front of the ***`self`*** shorthand to indicate this method
> borrows the ***`Self`*** instance, just as we did in ***`rectangle: &Rectangle`*** .
> Method can take ownership of ***`self`*** , borrow ***`self`*** immutably as we've done here,
> or borrow ***`self`*** mutably, just as they can any other parameter.

> We've chosen ***`&self`*** here for the same reson we used ***`&Rectangle`*** in the function version:
> we don't want to take ownership and we just want to read the data in the struct, not write to it.
> If we wanted to change the instance that we've called the method on as part of what the method does,
> we'd use ***`&mut self`*** as the first parameter.

#### Methods with More Parameters

<details>
<summary></summary>

``` rust

```

</details>

<details>
<summary>results</summary>

``` rust

```

</details>

#### Associated Functions

<details>
<summary></summary>

``` rust

```

</details>

<details>
<summary>results</summary>

``` rust

```

</details>

<br>

> **Summmary**
> Structs let you create custom types that are meaningful for your domain.
> By using structs, you can keep associated pieces of data connected to each otjer and
> name each piece to make your code clear.
> In ***`Impl`*** blocks, you can define functions that are associated with your type, and
> methods are a kind of associated funvctio that let you specify the behavior that instances of your structs have.

## Chapter 6

### Enums and Pattern Matching

> In this chapter we’ll look at enumerations, also referred to as enums. Enums allow you to 
> define a type by enumerating its possible variants. First, we’ll define and use an enum to show 
> how an enum can encode meaning along with data. Next, we’ll explore a particularly useful enum, 
> called Option, which expresses that a value can be either something or nothing. Then we’ll look 
> at how pattern matching in the match expression makes it easy to run different code for 
> different values of an enum. Finally, we’ll cover how the if let construct is another 
> convenient and concise idiom available to handle enums in your code.

> Enums are a feature in many languages, but their capabilities differ in each language. Rust’s 
> enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.

#### Dfining an Enum

> Enums are a way of defining custom data types in a different way than you do with structs.

## Chapter 7

### Managing Growing Projects with Packages, Crates and Modules


