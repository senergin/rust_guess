use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // ownership_test();

    println!("Guess the number!");
    println!("Please input your guess (1-100):");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    
    loop {
        let mut guess = String::new();  
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}

fn ownership_test() {
    let s = String::from("hello");  // s comes into scope

    println!("{}", s);
    takes_ownership(s);             // s's value moves into the function...
    // println!("{}", s);                                // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    println!("{}", x);
    makes_copy(x);                  // x would move into the function,
    println!("{}", x);                                // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
