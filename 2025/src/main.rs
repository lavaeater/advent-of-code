use clap::Parser;
use std::fs;

mod lock;
mod products;

use lock::*;
use products::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,
}

fn main() {
    let mut lock = Lock::new(50, 99);
    let args = Args::parse();
    match fs::read_to_string(args.file) {
        Ok(text) => {
            let lines = text.lines();
            for (i,line) in lines.enumerate() {
                if let Some(dir) = line.get(0..1)
                    && let Some(l) = line.get(1..)
                    && let Ok(ll) = l.parse::<i32>()
                {
                    let turn = if dir == "L" {
                        Turn::Left(ll)
                    } else {
                        Turn::Right(ll)
                    };
                    print!("{i} ");
                    lock.turn(turn);
                }
            }
            println!("The answer is {}", lock.number_of_zeroes);
        }
        Err(err) => {
            println!("${err}");
        }
    }
}


