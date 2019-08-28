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
