use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


// Function to test
fn compare_guess(guess: u32, secret_number: u32) -> Ordering {
    guess.cmp(&secret_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_guess_less() {
        assert_eq!(compare_guess(50, 100), Ordering::Less);
    }

    #[test]
    fn test_compare_guess_greater() {
        assert_eq!(compare_guess(100, 50), Ordering::Greater);
    }

    #[test]
    fn test_compare_guess_equal() {
        assert_eq!(compare_guess(50, 50), Ordering::Equal);
    }
}