fn main() {
    let num_to_convert = "10000000000000000000000000000101";
    for radix in 2..11 {
        let number: u64 = u64::from_str_radix(&num_to_convert, radix).expect(&format!("Couldn't convert {} with radix {}", num_to_convert, radix));
        println!("{}", number);
    }
}
