use std::io;

fn main() {
    let mut choice;
    
    loop {
        println!("Temperature Converter.");
        println!("a) Celcius to Fahrenheit.");
        println!("b) Fahrenheit to Celcius.");
        println!("c) quit program.");
        println!("Enter your choice: ");

        choice = user_choice();

        if choice == "a" {
            celcius_to_fahrenheit();
        }
        else if choice == "b" {
            fahrenheit_to_celcius();
        }
        else if choice == "c" {
            break;
        }
        else {
            println!("Please enter a, b or c.");
        }
    }
}

fn user_choice() -> String {
    let mut input = String::new();
    
    // Get user input (String) and assign it to input variable.
    io::stdin().read_line(&mut input)
       .expect("Failed read line.");
    
    // trim() result is &str, convert it back to_string().
    input.trim().to_string()
}

fn float_input() -> f32 {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
        .expect("Failed read line.");
    
    // parse String input to float.
    let num: f32 = user_input.trim().parse()
        .expect("Please enter a number");

    num 
}

fn celcius_to_fahrenheit() {
    println!("Enter celcius degree: ");

    let c = float_input();

    let f: f32 = (c * 1.8) + 32.;

    println!("{}", format!("{:.*} Celcius is {:.*} Fahrenheit.", 1, c, 1, f));
    println!("---------------");
}

fn fahrenheit_to_celcius() {
    println!("Enter fahrenheit degree: ");
    
    let f = float_input();

    let c: f32 = (f - 32.) / 1.8;

    println!("{}", format!("{:.*} Fahrenheit is {:.*} Celcius.", 1, f, 1, c ));
    println!("---------------");
}
