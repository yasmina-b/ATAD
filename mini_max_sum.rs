use std::io::{self, BufRead};

fn miniMaxSum(arr: &[i64]) {
    // Find the sum of all elements
    let total_sum: i64 = arr.iter().sum();

    // Find the minimum and maximum values in the array
    let min_value = *arr.iter().min().unwrap();
    let max_value = *arr.iter().max().unwrap();

    // The minimum sum is the total sum minus the largest number
    let min_sum = total_sum - max_value;
    // The maximum sum is the total sum minus the smallest number
    let max_sum = total_sum - min_value;

    // Print the results
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
