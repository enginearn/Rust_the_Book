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

``` rust
 let x = 2.0; // f64

let y: f32 = 3.0; // f32
```

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
   Compiling functions v0.1.0 (C:\Users\nagar\Development\Rust\projects\functions)
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
   Compiling functions v0.1.0 (C:\Users\nagar\Development\Rust\projects\functions)
   Finished dev [unoptimized + debuginfo] target(s) in 1.43s
     Running `C:\Users\nagar\Development\Rust\projects\functions\target\debug\functions.exe`
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
   Compiling functions v0.1.0 (C:\Users\nagar\Development\Rust\projects\functions)
   Finished dev [unoptimized + debuginfo] target(s) in 1.22s
     Running `C:\Users\nagar\Development\Rust\projects\functions\target\debug\functions.exe`
The value of x is 6
```

</details>

#### Comments

There is nothing special.

#### Control Flow

<details>
<summary></summary>

``` 

```

</details>
