use crate::Turn::{Left, Right};
use clap::Parser;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,
}

pub enum Turn {
    Left(i32),
    Right(i32),
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

pub struct Lock {
    pub start_position: i32,
    pub current_position: i32,
    pub lock_max: i32,
    pub number_of_zeroes: i32,
}

impl Lock {
    pub fn new(start: i32, max: i32) -> Self {
        Self {
            start_position: start,
            current_position: start,
            lock_max: max,
            number_of_zeroes: 0,
        }
    }
    
    pub fn reset(self: &mut Lock) {
        self.current_position = self.start_position;
        self.number_of_zeroes = 0;
    }
    
    pub fn rotate_right(self: &mut Lock) -> i32 {
        self.current_position += 1;
        if self.current_position > self.lock_max {
            self.current_position = 0;
        }
        self.current_position
    }

    pub fn rotate_left(self: &mut Lock) -> i32 {
        self.current_position -= 1;
        if self.current_position < 0 {
            self.current_position = self.lock_max;
        }
        self.current_position
    }

    pub fn rotate_n_right(self: &mut Lock, n: i32) -> i32 {
        let start = self.current_position;
        let mut ret = 0;
        for _i in 0..n {
            ret = self.rotate_right();
            if ret == 0 {
                self.number_of_zeroes +=1;
            }
        }
        ret
    }

    pub fn rotate_n_left(self: &mut Lock, n: i32) -> i32 {
        let start = self.current_position;
        let mut ret = 0;
        for _i in 0..n {
            ret = self.rotate_left();
            if ret == 0 {
                self.number_of_zeroes +=1;
            }
        }
        ret
    }

    pub fn turn(self: &mut Lock, t: Turn)-> i32 {
        match t {
            Left(ticks) => self.rotate_n_left(ticks),
            Right(ticks) => self.rotate_n_right(ticks),
        }
    }
}
