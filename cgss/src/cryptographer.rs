use serialize::base64::{self, ToBase64};
use rand::{OsRng, Rng};
use itertools::Itertools;

pub fn decode(data: String) -> String {
    if data.len() < 4 {
        return data;
    }
    let size = usize::from_str_radix(&data[..4], 16).unwrap();
    let output: String = data[6..].chars().step(4).take(size)
                        .map(|c| ((c as u8)-10) as char).into_iter().collect();
    output
}

pub fn generate_key_string() -> String {
    // Attempt to grab RNG pool
    let mut rng = match OsRng::new() {
        Ok(g) => g,
        Err(e) => panic!("Couldn't load the operating system's RNG pool: {}", e)
    };

    let mut raw_key = [0u8; 24];
    rng.fill_bytes(&mut raw_key);
    let key = raw_key.to_base64(base64::STANDARD);
    key
}

