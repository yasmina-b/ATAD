use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn flippingMatrix(matrix: &[Vec<i32>]) -> i32 {
    let n = matrix.len() / 2; // Size of the quadrant
    let mut max_sum = 0;

    for i in 0..n {
        for j in 0..n {
            // Determine the maximum value from the four candidates
            let candidates = [
                matrix[i][j],
                matrix[i][2 * n - 1 - j],
                matrix[2 * n - 1 - i][j],
                matrix[2 * n - 1 - i][2 * n - 1 - j],
            ];
            max_sum += *candidates.iter().max().unwrap();
        }
    }
    max_sum
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let mut matrix: Vec<Vec<i32>> = Vec::with_capacity((2 * n) as usize);

        for i in 0..(2 * n) as usize {
            matrix.push(Vec::with_capacity((2 * n) as usize));

            matrix[i] = stdin_iterator.next().unwrap().unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect();
        }

        let result = flippingMatrix(&matrix);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
