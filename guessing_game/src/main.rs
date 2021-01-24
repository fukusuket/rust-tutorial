extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let s_num = rand::thread_rng().gen_range(1, 101);
    println!("you guessed: {}", s_num);
    
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line.");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&s_num) {
            Ordering::Less => println!("A"),
            Ordering::Greater => println!("B"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }   
}
