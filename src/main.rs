use std::io::{self,Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..=100);
  //println!("The secret number is: {secret_number}");

  loop {
    let mut guess = String::new();

    print!("Please input your guess: ");
    let _ = io::stdout().flush();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!\n"),
      Ordering::Greater => println!("Too big!\n"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}
