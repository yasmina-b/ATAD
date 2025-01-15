use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn findDigits(n: i32) -> i32 {
    let mut count = 0;
    let mut num = n;
    while num > 0 {
        let digit = num % 10;
        if digit != 0 && n % digit == 0 {
            count += 1;
        }
        num /= 10;
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = findDigits(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
