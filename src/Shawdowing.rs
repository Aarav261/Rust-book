use std::io;

fn main() {
    let x = 5; 
    let x = x + 1; // shadowing the previous x

    {
        let x = x * 2; // shadowing the previous x in a new scope
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x in the outer scope is: {}", x);
}