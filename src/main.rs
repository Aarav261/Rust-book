use std::io;
use std::cmp::Ordering;
#[path = "Shawdowing.rs"]
mod shadowing;
use crate::shadowing::shadowing;
use rand::Rng;
use std::fmt::Display;
mod Loops;
use crate::Loops::loops;

fn main(){

    loops();
    
    println!("Guess the number!");

    let result = shadowing();
    println!("Result: {}", result);

    let secret_number = rand::thread_rng().gen_range(1..=100); //u32 is the default type for numbers in Rust, so we don't need to specify it here
    println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");

    let mut guess = String::new(); 

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    }; // shadowing
    //secret number should also be a u32, so we can compare them
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){ 
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break; }
    }
}  

}


