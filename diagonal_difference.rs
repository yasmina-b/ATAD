use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();  // The size of the square matrix (n x n)
    
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;
    
    // Traverse the matrix to calculate both diagonals
    for i in 0..n {
        primary_diagonal_sum += arr[i][i];           // sum of primary diagonal
        secondary_diagonal_sum += arr[i][n - 1 - i]; // sum of secondary diagonal
    }
    
    // Calculate the absolute difference between the two sums
    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
