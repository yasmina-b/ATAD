use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn maxMin(k: i32, arr: &[i32]) -> i32 {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_unstable(); // Sort the array

    let k = k as usize;
    let mut min_difference = i32::MAX;

    // Find the smallest difference between max and min in each subarray of size k
    for i in 0..=(sorted_arr.len() - k) {
        let current_difference = sorted_arr[i + k - 1] - sorted_arr[i];
        if current_difference < min_difference {
            min_difference = current_difference;
        }
    }
    min_difference

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let k = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let arr_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        arr.push(arr_item);
    }

    let result = maxMin(k, &arr);

    writeln!(&mut fptr, "{}", result).ok();
}
