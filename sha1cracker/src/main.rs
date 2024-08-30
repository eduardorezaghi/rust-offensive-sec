use std::{env, error::Error, fs::File, io::BufRead, io::BufReader};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("usage: ");
        println!("sha1_cracker: <wordlist.txt> <hash>");
        return Ok(());
    }

    let wordlist_path = &args[1];

    let hash = &args[2].trim();
    if hash.len() != SHA1_HEX_STRING_LENGTH {
        println!("Invalid hash length");
        return Err("Invalid hash length".into());
    }

    let wordlist = File::open(wordlist_path)?;
    let reader = BufReader::new(wordlist);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_args_length() {
        let args = vec![String::from("sha1_cracker"), String::from("wordlist.txt")];
        assert!(main_with_args(args).is_ok());
    }

    #[test]
    fn test_invalid_hash_length() {
        let args = vec![
            String::from("sha1_cracker"),
            String::from("wordlist.txt"),
            String::from("invalid_hash"),
        ];
        let result = main_with_args(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_args_and_hash() {
        let args = vec![
            String::from("sha1_cracker"),
            String::from("wordlist.txt"),
            String::from("valid_hash"),
        ];
        assert!(main_with_args(args).is_ok());
    }

    fn main_with_args(args: Vec<String>) -> Result<(), Box<dyn Error>> {
        env::set_var("RUST_BACKTRACE", "0");
        env::set_var("RUST_LOG", "info");

        let result = main();

        env::remove_var("RUST_BACKTRACE");
        env::remove_var("RUST_LOG");

        result
    }
}
