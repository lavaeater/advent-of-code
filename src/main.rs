fn main() {
    println!("Hello, world!");
}

pub struct Lock {
  pub current_position: i32,
  pub lock_max: i32,
}

impl Lock {
  pub fn move_forward(mut self: Lock) -> i32 {
    self.current_position += 1;
    if self.current_position > self.lock_max {
      self.current_position = 0;
    }
    self.current_position
  }

  pub fn rotate_righ(mut self: Lock) -> i32 {
    self.current_position -= 1;
    if self.current_position < 0 {
      self.current_position = self.lock_max;
    }
    self.current_position
  }
  
}
