use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeConversion(s: &str) -> String {
    // Extract the hour, minute, second, and period (AM/PM)
    let hour = &s[0..2]; // First two characters: hour
    let minute = &s[3..5]; // Characters 3-4: minute
    let second = &s[6..8]; // Characters 6-7: second
    let period = &s[8..];  // Last two characters: AM/PM

    // Convert hour to an integer
    let mut hour_int: i32 = hour.parse().unwrap();

    // Apply conversion rules
    if period == "AM" {
        if hour_int == 12 {
            // Midnight case: 12AM -> 00
            hour_int = 0;
        }
    } else if period == "PM" {
        if hour_int != 12 {
            // Afternoon case: Add 12 to the hour
            hour_int += 12;
        }
    }
    // Format the hour, minute, and second into a 24-hour time string
    format!("{:02}:{:02}:{:02}", hour_int, minute.parse::<i32>().unwrap(), second.parse::<i32>().unwrap())
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
