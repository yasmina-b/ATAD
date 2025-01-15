use std::io::{self, BufRead};

fn extraLongFactorials(n: i32) {
    let mut result = vec![1]; // Initialize result as 1
    for i in 2..=n {
        multiply(&mut result, i); // Multiply result by each number from 2 to n
    }
    for digit in result.iter().rev() {
        print!("{}", digit); // Print result in reverse order
    }
    println!(); // Ensure there's a newline at the end
}

fn multiply(result: &mut Vec<i32>, num: i32) {
    let mut carry = 0;
    for digit in result.iter_mut() {
        let product = *digit * num + carry;
        *digit = product % 10; // Store the last digit
        carry = product / 10;  // Store the carry
    }
    while carry > 0 {
        result.push(carry % 10); // Handle any remaining carry
        carry /= 10;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    extraLongFactorials(n);
}
