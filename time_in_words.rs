use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeInWords(h: i32, m: i32) -> String {
    let time_words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen", "twenty", "twenty one", "twenty two", "twenty three", "twenty four",
        "twenty five", "twenty six", "twenty seven", "twenty eight", "twenty nine"
    ];

    if m == 0 {
        return format!("{} o' clock", time_words[h as usize]);
    } else if m == 15 {
        return format!("quarter past {}", time_words[h as usize]);
    } else if m == 30 {
        return format!("half past {}", time_words[h as usize]);
    } else if m == 45 {
        return format!("quarter to {}", time_words[((h % 12) + 1) as usize]);
    } else if m < 30 {
        return format!(
            "{} minute{} past {}",
            time_words[m as usize],
            if m == 1 { "" } else { "s" },
            time_words[h as usize]
        );
    } else {
        let minutes_to_next_hour = 60 - m;
        return format!(
            "{} minute{} to {}",
            time_words[minutes_to_next_hour as usize],
            if minutes_to_next_hour == 1 { "" } else { "s" },
            time_words[((h % 12) + 1) as usize]
        );
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let h = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = timeInWords(h, m);

    writeln!(&mut fptr, "{}", result).ok();
}
