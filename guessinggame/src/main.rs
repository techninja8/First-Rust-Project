use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        let mut value = String::new();
        
        let secret_number = rand::thread_rng().gen_range(1..10);

        io::stdin()
            .read_line(&mut value)
            .unwrap();

        let value: u32 = match value.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        match value.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too Small\n");
            },
            Ordering::Equal => {
                println!("You Won\n");
                break; // Stop the game once the player is correct
            },
            Ordering::Greater => {
                println!("Too Big!\n");
            }
        }
        // println!("Your value is {value}");
        println!("Secret number was {secret_number}.");
    }
}
