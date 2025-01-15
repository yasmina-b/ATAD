use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn findMedian(arr: &[i32]) -> i32 {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_unstable(); // Sort the array in ascending order
    let mid = sorted_arr.len() / 2;
    sorted_arr[mid] // Return the middle element
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = findMedian(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
