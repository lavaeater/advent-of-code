use Turn::*;

pub enum Turn {
    Left(i32),
    Right(i32),
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
        let mut ret = 0;
        for _i in 0..n {
            ret = self.rotate_right();
            if ret == 0 {
                self.number_of_zeroes += 1;
            }
        }
        ret
    }

    pub fn rotate_n_left(self: &mut Lock, n: i32) -> i32 {
        let mut ret = 0;
        for _i in 0..n {
            ret = self.rotate_left();
            if ret == 0 {
                self.number_of_zeroes += 1;
            }
        }
        ret
    }

    pub fn turn(self: &mut Lock, t: Turn) -> i32 {
        match t {
            Left(ticks) => self.rotate_n_left(ticks),
            Right(ticks) => self.rotate_n_right(ticks),
        }
    }
}
