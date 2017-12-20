use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line 1");
    let total_cases: u32 = line.trim().parse().expect("not a number.");
    for case in 1..total_cases+1 {
        line.truncate(0);
        io::stdin().read_line(&mut line).expect("failed to read line");
        let lastword = thelastword(line.trim());
        println!("Case #{}: {}", case, lastword);
    }
}

fn thelastword(input: &str) -> String {
    let letters: Vec<char>= input.chars().collect();
    let mut lastword = String::new();
    let mut first_letter = ' ';
    for letter in letters {
        if letter >= first_letter {
            lastword.insert(0, letter);
            first_letter = letter;
        } else {
            lastword.push(letter);
        }
    }
    return lastword;
}

