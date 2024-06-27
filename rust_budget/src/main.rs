use std::{arch::x86_64::_MM_EXCEPT_INEXACT, io, process::exit};

fn main() {
    println!("Welcome to rustBudget, a program to help you not go broke!");
    bill_loop();
}

fn bill_loop() {
    let n = 0;
    while n < 1 {
        let mut input = String::new(); //mutable string
        println!("What would you like to do?\n1. Add a bill\n2. Modify a bill\n3. Exit");
        io::stdin()
            .read_line(&mut input) //reads input
            .expect("Incorrect, please try again");
        input = input.trim().to_string(); //trims it

        if input.eq("1") {
            //if input is equal to 1
            take_bill();
        }
        if input.eq("2") {
            //if input is equal to 2
            add_bill_to_bills();
        }
        if input.eq("3") {
            exit(1);
        }
    }
}

fn take_bill() {
    let mut input = String::new(); //creating a mutable String called input
    println!("Please enter a bill to add to your budget"); //outputting a test
    io::stdin() //waiting for an input
        .read_line(&mut input) //reading input
        .expect("Failed to read line"); //error code
    println!("Debugg: {}", input.trim()); //outputting input
}

fn add_bill_to_bills() {
    //this is for an array
}
