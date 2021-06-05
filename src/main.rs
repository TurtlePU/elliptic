use std::io::{Read, stdin};

use rand::thread_rng;

use crate::ecies::ec_encryptor;

pub mod algebra;
pub mod bytes;
pub mod encryption;

mod dlies;
mod ecies;

fn main() {
    let ec = ec_encryptor();
    let (enc, dec) = ec.generate_keys(&mut thread_rng());
    let mut text = Vec::new();
    stdin().lock().read_to_end(&mut text).unwrap();
    let text = String::from_utf8(text).unwrap();
    println!("{}", text.clone());
    let cipher = enc.encrypt(&mut thread_rng(), text);
    println!("{}", cipher.clone());
    let text = dec.decrypt(cipher).unwrap();
    println!("{}", text);
}
