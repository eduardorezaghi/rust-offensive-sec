use std::{
    env,
    error::Error, fs::File, io::BufRead,
};

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
    let reader = std::io::BufReader::new(wordlist);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

