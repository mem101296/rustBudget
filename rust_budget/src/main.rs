use std::{arch::x86_64::_MM_EXCEPT_INEXACT, io, process::exit};
use ndarray::prelude::*;
use ndarray::{ArrayBase, Array, Array3};

fn main() {
    //let mut bills = Array::<f64, _>::zeros((3,4,5).f());
    println!("Welcome to rustBudget, a program to help you not go broke!");
    //https://github.com/rust-ndarray/ndarray/blob/master/README-quick-start.md
    //bills[[2,2,2]] = 15.0; //modify array at 2,2,2
    //println!("{}", bills[[2,2,2]]); //output array at 2,2,2
    //TODO Figure out how to pass arrays between functions
    bill_loop();
}

fn bill_loop() {
    let n = 0;
    let mut bills = Array::<f64, _>::zeros((3,4,5).f());
    while n < 1 {
        let mut input = String::new(); //mutable string
        println!("What would you like to do?\n1. Add a bill\n2. Modify a bill\n3. Exit");
        io::stdin()
            .read_line(&mut input) //reads input
            .expect("Incorrect, please try again");
        input = input.trim().to_string(); //trims it

        if input.eq("1") {
            //if input is equal to 1
            take_bill(bills.clone());
            //bills[[2,2,2]] = 15;
            println!("{}", bills[[2,2,2]]);
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

fn take_bill<T>(mut A: Array::<f64, T>){
    let mut input = String::new(); //creating a mutable String called input
    println!("Please enter a bill to add to your budget"); //outputting a test
    io::stdin() //waiting for an input
        .read_line(&mut input) //reading input
        .expect("Failed to read line"); //error code
    println!("Debugg: {}", input.trim()); //outputting input
    A[[2,2,2]] = 16.0;
    
}

fn add_bill_to_bills() {
    //this is for an array
}
