extern crate primal;

use std::io;
use primal::is_prime;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line 1");
    let total_cases: u32 = line.trim().parse().expect("not a number.");
    for case in 1..total_cases+1 {
        line.truncate(0);
        io::stdin().read_line(&mut line).expect("failed to read line");
        println!("Case #{}:", case);
        let params: Vec<&str> = line.trim().split(' ').collect();
        let jamcoin_length: u32 = params[0].parse().expect("Couldn't parse required jamcoin length.");
        let jamcoin_count: usize = params[1].parse().expect("Couldn't parse number of jamcoins to generate.");
        let mut jamcoins_found: usize = 0;
        for inner_10 in 0..2u64.pow(jamcoin_length - 2) {
            let coin_pre_binary: String = format!("{:b}", inner_10);
            let missing_length = jamcoin_length as usize - 2 - coin_pre_binary.len() as usize;
            let coin_binary = format!("1{}{}1", (0..missing_length).map(|_| "0").collect::<String>(), coin_pre_binary);
            let mut found_prime = false;
            let mut values_per_radix = Vec::new();
            println!("\nchecking {}", coin_binary);
            for radix in 2..11u32 {
                println!("{:?}", radix);
                let number_to_check: u64 = u64::from_str_radix(&coin_binary, radix).expect("Number too big?");
                values_per_radix.push(number_to_check);
                //println!("{}", number_to_check);
                if is_prime(number_to_check) {
                    //println!("is prime");
                    found_prime = true;
                    break;
                }
            }
            if found_prime == false {
                let nontrivial_divisors: Vec<_> = values_per_radix.iter().map(|&x| get_nontrivial_divisor(x).to_string()).collect::<Vec<_>>();
                println!("{} {}", coin_binary, nontrivial_divisors.join(" "));
                jamcoins_found += 1;
            }
            if jamcoins_found == jamcoin_count {
                break;
            }
        }
    }
}

fn get_nontrivial_divisor(n: u64) -> u64 {
    for f in 2..n {
        if n % f == 0 {
            return f;
        }
    }
    return 0;
}
