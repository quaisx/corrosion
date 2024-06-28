// File: mod.rs
// Author: Qua Is X
// Date: 2024-06-21
// Description: Fibonacci sequence generator.

// License: The Unlicense.
// This code is licensed under The Unlicense.

use std::iter::Iterator;

// Fibonacci sequence generator structure.
pub struct Fibonacci (u32, u32);

////////////////////////////////
impl Fibonacci {
   // Fibonacci sequence generator constructor.
   pub fn new() -> Self {
      Fibonacci (0, 1)
   }
}

// Fibonacci sequence generator Iterator implementation
impl Iterator for Fibonacci {
    // Iterator Self::Item
    type Item = u32;

    // Then `next` value in the sequence is wrapped in `Option<Self::Item>`
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.0;
        self.0 = self.1;
        self.1 = tmp + self.1;
        Some(tmp)
    }
}

// fibonacci - returns a new Fibonacci sequence generator
pub fn fibonacci() -> Fibonacci {
    Fibonacci::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_0() {
        let mut test = fibonacci();
        assert_eq!(test.next().unwrap(), 0u32);
    }

    #[test]
    fn it_works_1() {
        let test = fibonacci();
        assert_eq!(test.skip(1).next().unwrap(), 1u32);
    }

    #[test]
    fn it_works_2() {
      let test = fibonacci();
      assert_eq!(test.skip(2).next().unwrap(), 1u32);
    }

    #[test]
    fn it_works_array() {
      let test_array = [0u32, 1, 1, 2, 3, 5, 8, 13];
      let mut fib = fibonacci();
      for v in test_array.iter() {
         assert_eq!(fib.next().unwrap(), *v);
      }
    }
}
