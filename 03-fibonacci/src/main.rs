use std::io;

// fibonacci function
// command line application that
// inputs a user input number n and
// outputs the nth fibonacci number

fn main() {
    println!("Fibonacci!");

    // ask user to input number in ternimal
    println!("Please input a number:");

    let mut input = String::new();

    // read input from terminal
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // convert input to integer
    let input: u32 = input.trim().parse().expect("Please type a number!");

    let mut a: i32 = 0;
    let mut b: i32 = 1;

    // loop that calculates and prints fibonacci numbers
    for _ in 0..input {
        let temp: i32 = a;
        a = b;
        b = temp + b;
        print!("{} ", a);
    }

    // print new line at end of program
    println!("");
}
