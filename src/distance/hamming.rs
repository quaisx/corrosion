// File: hamming.rs
// Author: Qua Is X
// Date: 2024-06-29
// Description: Calculate the hamming distance between two u32.

// License: The Unlicense.
// This code is licensed under The Unlicense.

pub fn hamming_distance(x: u32, y: u32) -> u32 {
    let mut dist = 0u32;
    let mut v = x ^ y;
    while v > 0 {
        dist = dist + 1;
        v = v & (v - 1);
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_0() {
        let res = hamming_distance(0, 0);
        assert_eq!(res, 0);
    }
    #[test]
    fn it_works_1() {
        let res = hamming_distance(0, 1);
        assert_eq!(res, 1);
    }
    #[test]
    fn it_works_2() {
        let res = hamming_distance(0, 2);
        assert_eq!(res, 1);
    }
    #[test]
    fn it_works_f() {
        let res = hamming_distance(0x0, 0xF);
        assert_eq!(res, 4);
    }
    #[test]
    fn it_works_ff() {
        let res = hamming_distance(0x0, 0xFF);
        assert_eq!(res, 8);
    }
}
