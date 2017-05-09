// bring input/output library into scope (from standard library)
// bring libraries into scope explicity with `use` keyword
use std::io;

// main function is entry point into program
fn main() {
    // println! is a macro
    println!("Guess the number I am thinking of");
    println!("Please enter a number: ");
    // the `let` keyword is used to create variables
    // in rust, variables are immutable by default, but the `mut` keyword allows mutability
    // :: -> association, `new` is associated function of `String` type (static method)
    let mut guess = String::new();
    // call the `stdin` method from the `io` library on first line of program
    // io::stdin() returns handle to standard input for terminal
    // .readline(str x) gets input from user and returns io::Result
    // string target of .readline(str x) must be mutable to accept value
    // & -> reference, and references are immutable by default, therefore need &mut
    io::stdin().read_line(&mut guess)
               .expect("Failed to read in line");
    // {} is a placeholder that holds a value in place (string interpolation)
    println!("You guess {}", guess);
    string_interpolation();
}

fn string_interpolation() {
    let x = "Joseph";
    let y = "Choi";
    // multiple string interpolation values can be used so long as they are in order
    println!("My full name is {} {}", x, y);
}
