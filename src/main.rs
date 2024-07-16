use ndarray::prelude::*;
use ndarray::{array, Array, ArrayView};
use std::{io, process::exit};

fn main() {
    let mut bills = Array::<f64, _>::zeros((3, 1, 5).f()); //Create emply 3d array 3 4x5 matrix - will change later
                                                           //TODO Make D1 String, D2 float, D3 float - D1 bill name, D2 bill amount, D3 bill date
    println!("Welcome to rustBudget, a program to help you not go broke!");
    //https://github.com/rust-ndarray/ndarray/blob/master/README-quick-start.md
    // main() function still "owns" bills, and this allows bill_loop() to modify it
    bill_loop(&mut bills);
    // we can continue to use bills in main after this point
}

fn bill_loop(bills: &mut Array<f64, Ix3>) {
    //allows the function to take the passed array
    let n = 0; //varaible to create infinite loop
               //let mut bills = Array::<f64, _>::zeros((3,4,5).f());
    while n < 1 {
        let mut input = String::new(); //mutable string
        println!("What would you like to do?\n1. Add a bill\n2. Modify a bill\n3. Exit");
        io::stdin()
            .read_line(&mut input) //reads input
            .expect("Incorrect, please try again"); //error msg
        input = input.trim().to_string(); //trims it to a string

        if input.eq("1") {
            //if input is equal to 1
            add_bill(bills); //passes bills to
            bills[[2, 0, 2]] = 15.0;
            println!("2,0,2: {}", bills[[2, 0, 2]]); //used for debugging - to check if changes applied
        }
        if input.eq("2") {
            //if input is equal to 2
            modify_bills(bills);
        }
        if input.eq("3") {
            println!("\n\ntotal = {}", bills.sum()); //change to a function that outputs the entire array
            exit(1);
        }
    }
}

fn add_bill(bills: &mut Array<f64, Ix3>) {
    let mut input = String::new(); //creating a mutable String called input
    println!("Please enter a bill to add to your budget"); //outputting a test
    io::stdin() //waiting for an input
        .read_line(&mut input) //reading input
        .expect("Failed to read line"); //error code
    let input_float: f64 = input.trim().parse().expect("Please enter a valid float"); //ensures input is a float
    let input_float_2 = input_float + 1.0; //used for convience and debugging
    let input_float_3 = input_float + 2.0; //used for convience and debugging
    println!("Debug: {}", input.trim()); //outputting input

    //index
    let (d, r, c) = bills.dim();
    println!("Dimension: {} Row: {}, Collumn {}", d, r, c);
    //append at end
    let input_bills = ArrayView::from(&[0.; 0]);
    bills.push(Axis(1), input_bills).unwrap();
    //insert input into array
}

fn modify_bills(bills: &mut Array<f64, Ix3>) {
    //this is for an array
}
