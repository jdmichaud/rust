use std::result::Result;
use std::u32;

pub fn nth(n: u32) -> Result<u32, &'static str> {
  pub fn is_prime(v: u32) -> bool {
    for div in 2..v {
      if v % div == 0 {
        return false;
      }
    }
    return true;
  }

  return match n {
    0 => Err("Input error"),
    1 => Ok(2),
    _ => {
      let mut count = 0;
      for m in 3..u32::MAX {
        count += if is_prime(m) { 1 } else { 0 };
        if count >= n - 1 {
          return Ok(m);
        }
      }
      return Err("Unexpected error");
    }
  }
}
