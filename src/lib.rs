mod fibonacci;
mod vector;
mod distance;

pub use fibonacci::fibonacci;
pub use vector::vector_join;
pub use distance::hamming::hamming_distance;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
