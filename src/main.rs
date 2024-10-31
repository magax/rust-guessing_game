use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nGuess the number!");

    let secret_nb = rand::thread_rng().gen_range(0..=255);

    //[CHEAT CODE] Uncomment the following line to show the generated number:
    //println!("Secret number: {secret_nb}");

    loop {
        println!("\nEnter your guess:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read number");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Only inclusive numbers between 0 and 255 will work!");
                continue;
            }
        };

        match guess.cmp(&secret_nb) {
            Ordering::Less => println!("⬆️ Too small!"),
            Ordering::Greater => println!("⬇️ Too big!"),
            Ordering::Equal => {
                println!("🎉 You win!");
                break;
            }
        }
    }
}
