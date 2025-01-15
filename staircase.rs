use std::io::{self, BufRead};

fn staircase(n: i32) {
    for i in 1..=n {
        // Print the spaces for the current row
        for _ in 0..(n - i) {
            print!(" ");
        }
        // Print the hashes for the current row
        for _ in 0..i {
            print!("#");
        }
        // Move to the next line after each row
        println!();
    }
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}
