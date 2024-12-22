mod monkey;

use std::fs;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;

use crypto::md5::Md5;
use crypto::digest::Digest;

use monkey::Monkey;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// The string we need to try and match
    #[arg(required = true)]
    input_md5sum: String,

    /// The file to write to
    #[arg(short, long, default_value = "output.txt")]
    output: String
}

fn check_str(mut hasher: Md5, guess: &str, comp_sum: &str) -> bool {
    hasher.reset();
    hasher.input_str(guess);
    hasher.result_str() == comp_sum
}

fn main() {
    let args = Args::parse();
    let output_file = args.output;
    println!("Attempting to find input that matches {}", args.input_md5sum);
    let hasher = Md5::new();

    // There are 191726 in Hamlet. To give the monkey a chance
    // they will only be required to guess up to this many characters
    // let character_limit: u32 = 191727;
    let character_limit = 3;
    let mut monkey = Monkey::new(character_limit as u64);
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    loop {
        let guess = monkey.smack_typewriter();
        if check_str(hasher, &guess, &args.input_md5sum) {
            let elapsed_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start_time;
            println!("Found match! Took {} monkies approximately {} milliseconds", monkey.guess_count, elapsed_time.as_millis());
            fs::write(&output_file, guess).expect(
                &format!("Unable to save ouput to {}! How dare you", &output_file)
            );
            exit(0);
        }
    }
}
