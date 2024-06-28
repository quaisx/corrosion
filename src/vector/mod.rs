// File: mod.rs
// Author: Qua Is X
// Date: 2024-06-21
// Description: Fibonacci sequence generator.

// License: The Unlicense.
// This code is licensed under The Unlicense.

////////////////////////////////
/// vector_join - join two vectors
pub fn vector_join<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T>
where
    T: std::clone::Clone,
{
    vec1.into_iter().chain(vec2.into_iter()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vector_join() {
        let v1 = vec![1u32, 2, 3];
        let v2 = vec![4u32, 5, 6];
        let res: Vec<_> = vector_join(v1, v2);
        assert_eq!(res.len(), 6);
    }
}
