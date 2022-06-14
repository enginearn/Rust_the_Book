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
Compiling hello_cargo v0.1.0 (C:\Users\nagar\Development\Rust\projects\hello_cargo)
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
Checking hello_cargo v0.1.0 (C:\Users\nagar\Development\Rust\projects\hello_cargo)
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
Compiling guessing_game v0.1.0 (C:\Users\nagar\Development\Rust\projects\guessing_game)
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
   Compiling guessing_game v0.1.0 (C:\Users\nagar\Development\Rust\projects\guessing_game)
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
 Documenting guessing_game v0.1.0 (C:\Users\nagar\Development\Rust\projects\guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 10.35s
     Opening C:\Users\nagar\Development\Rust\projects\guessing_game\target\doc\guessing_game\index.html
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
   Compiling guessing_game v0.1.0 (C:\Users\nagar\Development\Rust\projects\guessing_game)
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

