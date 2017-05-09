extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // println! is a macro
    println!("Guess the number I am thinking of");
    let lower_range = 1;
    let upper_range = 100;
    let secret_number = rand::thread_rng().gen_range(lower_range, upper_range + 1);
    let secret_string = secret_number.to_string();
    println!("Please guess a number between {} and {}: ", lower_range, upper_range);
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
    println!("You guessed {}", guess);

    match guess.cmp(&secret_string) {
        Ordering::Less    => println!("Your guess was too small..."),
        Ordering::Greater => println!("Your guess was too large..."),
        Ordering::Equal   => println!("Your guess was correct, you win!")
    }

    println!("The secret number was {}", secret_number);
    string_interpolation();
}

fn string_interpolation() {
    let x = "Joseph";
    let y = "Choi";
    // {} is a placeholder that holds a value in place (string interpolation)
    // multiple string interpolation values can be used so long as they are in order
    println!("My full name is {} {}", x, y);
}
