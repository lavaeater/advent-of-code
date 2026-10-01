use clap::{ Parser, Subcommand, ValueEnum };
use std::{fs, io};

mod lock;
mod products;

use lock::*;
use products::*;

use core::str::Lines;

#[derive(Debug, Clone, Parser, ValueEnum)]
enum Action {
    Lock,
    InvalidIds
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    action: Action,
    #[arg(short, long)]
    file: String,
}

fn file_to_strings(path: &String)-> Result<Vec<String>, String> {
    match fs::read_to_string(path) {
        Ok(s) => Ok(s.lines().map(|s|s.to_owned()).collect()),
        Err(e) => Err(format!("Could not parse input file: {}", e))
    }
}

fn unlock(path: String) {
    match file_to_strings(&path) {
        Ok(lines) => {
            let mut lock = Lock::new(50, 99);
            for (i,line) in lines.iter().enumerate() {
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

fn invalid_ids(path: String) {
    match fs::read_to_string(&path) {
        Ok(s) => {
            let sum: i64 = s
            .split(",")
            .map(|r|generate_range(r))
            .flatten()
            .filter(|s|is_invalid(s))
            .map(|s|s.parse::<i64>().map_or_default(|i| i))
            .sum();
/*            
            let sum: i64 = s.split(",")
            .map(|r| generate_range(r))
            .flatten()
            .filter(|s|is_invalid(s))
            .map(|s|s.parse::<i64>().map_or_default(|i|i))
            .sum();
            */
            println!("The sum of all invalid ids is: {sum}!");
        },
        Err(e) => {println!("Something went wrong: {}", e);}
    }    
}

fn main() {
    let args = Args::parse();
    match args.action {
        Action::Lock => unlock(args.file),
        Action::InvalidIds => invalid_ids(args.file)
    }
}



