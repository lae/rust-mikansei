use serialize::base64::{self, FromBase64};

use cryptographer;
use cryptaes;

pub fn decrypt_user_id(enc_user_id: String) -> String {
    return cryptographer::decode(enc_user_id);
}

pub fn unpack(raw_data: String, udid: String) -> String {
    let data = raw_data.from_base64().ok().unwrap();
    let iv = cryptographer::decode(udid).replace("-", "");

    let key_index = data.len()-32;
    let (ciphertext, key) = data.split_at(key_index);
    println!("ciphertext={:?}\nkey={:?}", ciphertext, key);
    //let key: [u8; 32] = data[key_index..];
    //let ciphertext: Vec<u8> = data[..key_index];
    println!("{:?}", String::from_utf8(key.to_vec()).unwrap());
    //let plaintext = cryptaes::decrypt(ciphertext, key, &iv.as_bytes()).ok().unwrap();
    match cryptaes::decrypt(ciphertext, key, &iv.as_bytes()) {
        Ok(n) => println!("{:?}", n),
        Err(e) => println!("{:?}", e)
    };
   // println!("{:?}", plaintext);
    iv
}
