#![allow(dead_code)]


// --- Directions
// Given an array and chunk size, divide the array into many subarrays
// where each subarray is of length size
// --- Examples
// chunk([1, 2, 3, 4], 2) --> [[ 1, 2], [3, 4]]
// chunk([1, 2, 3, 4, 5], 2) --> [[ 1, 2], [3, 4], [5]]
// chunk([1, 2, 3, 4, 5, 6, 7, 8], 3) --> [[ 1, 2, 3], [4, 5, 6], [7, 8]]
// chunk([1, 2, 3, 4, 5], 4) --> [[ 1, 2, 3, 4], [5]]
// chunk([1, 2, 3, 4, 5], 10) --> [[ 1, 2, 3, 4, 5]]

fn solution_1(v: Vec<i32>, size: usize) -> Vec<Vec<i32>> {
    let mut chunks = vec![];
    let mut chunk = vec![];
    for (n, i) in v.iter().enumerate() {
        chunk.push(*i);
        if (n+1) % size == 0 || n == v.len() - 1{
            chunks.push(chunk.clone());
            chunk.clear();
        }
    }

    chunks
}

fn solution_2(v: Vec<i32>, size: usize) -> Vec<Vec<i32>> {
    let mut chunks = vec![];
    let mut i = 0;
    
    while i < v.len() {
        let end_index = if i + size > v.len() { v.len() } else { i + size }; //prevents out of bounds indexing
        chunks.push(v[i..end_index].to_vec());
        i += size;
    }

    chunks
}

pub fn chunked(v: Vec<i32>, size: usize) -> Vec<Vec<i32>> {
    // solution_1(v, size)
    solution_2(v, size)
}

#[cfg(test)]
mod tests {
    use crate::chunk::chunked;

    #[test]
    fn test_1() {
         let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
         assert_eq!(chunked(v, 2), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]);
    }

    #[test]
    fn test_2() {
         let v = vec![1, 2, 3];
         assert_eq!(chunked(v, 1), vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_3() {
         let v = vec![1, 2, 3, 4, 5];
         assert_eq!(chunked(v, 3), vec![vec![1, 2, 3], vec![4, 5]]);
    }

    #[test]
    fn test_4() {
         let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
         assert_eq!(chunked(v, 5), vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13]]);
    }
}
