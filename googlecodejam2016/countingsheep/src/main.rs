use std::io;
use std::collections::HashSet;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line 1");
    let total_cases: u32 = line.trim().parse().expect("not a number.");
    for case in 1..total_cases+1 {
        line.truncate(0);
        io::stdin().read_line(&mut line).expect("failed to read line");
        let last = sheep(line.trim());
        println!("Case #{}: {}", case, last);
    }
}

fn sheep(n_str: &str) -> String {
    let mut digits_seen = HashSet::new();
    let n: u32 = n_str.parse().expect("not a number.");
    for i in 1..101 {
        let cnum = i*n;
        let cnumstr = cnum.to_string();
        for digit_char in cnumstr.chars() {
            let digit: u32 = digit_char.to_digit(10).unwrap();
            if digits_seen.contains(&digit) == false {
                digits_seen.insert(digit);
            }
        }
        if digits_seen.len() == 10 {
            return cnumstr;
        }
    }
    return "INSOMNIA".to_owned();
}
