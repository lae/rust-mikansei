use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line 1");
    let total_cases: u32 = line.trim().parse().expect("not a number.");
    for case in 1..total_cases+1 {
        line.truncate(0);
        io::stdin().read_line(&mut line).expect("failed to read line");
        let flips = pancakeflips(line.trim());
        println!("Case #{}: {}", case, flips);
    }
}

fn pancakeflips(input: &str) -> usize {
    let mut flips: usize = 0;
    let mut stack: String = input.to_owned();
    let stack_length: usize = stack.len();
    loop {
        if !stack.contains("-") {
            // We're already fine with the original stack.
            break;
        }
        let _stack = stack.clone();
        let happy_bottom = _stack.trim_right_matches('+');
        let happy_bottom_len = stack_length - happy_bottom.len();
        let happy_top = _stack.trim_left_matches('+');
        let happy_top_len = stack_length - happy_top.len();
        let blank_top = _stack.trim_left_matches('-');
        let blank_top_len = stack_length - blank_top.len();
        if happy_bottom_len >= 1 {
            if happy_top_len >= 1 {
                stack = flip(&_stack, happy_top_len);
            } else {
                stack = flip(&_stack, blank_top_len);
            }
        } else {
            if blank_top_len >= 1 {
                stack = flip(&_stack, blank_top_len);
            } else {
                stack = flip(&_stack, happy_top_len);
            }
        }
        flips += 1;
    }
    return flips;
}

fn flip(input: &str, index: usize) -> String {
    let (top, bottom) = input.split_at(index);
    let flipped_top = top.chars().rev().collect::<String>().replace("-", "M").replace("+", "-").replace("M", "+");
    return format!("{}{}", flipped_top, bottom);
}
