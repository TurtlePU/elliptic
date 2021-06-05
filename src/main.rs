use std::io::{stdin, Read};

use clap::clap_app;
use dlies::zp_encryption;
use rand::thread_rng;

use crate::{
    attack::crack, ecges::ec_encryptor, encryption::extensions::PublicEncObject,
};

pub mod algebra;
pub mod bytes;
pub mod encryption;

mod attack;
mod dlies;
mod ecges;

fn main() {
    let matches = clap_app!(elliptic =>
        (version: "0.1")
        (author: "Pavel Sokolov <sokolov.p64@gmail.com>")
        (about: "Elliptic curves POC")
        (@arg CURVE: -c --curve conflicts_with[PRIME CRACK] "Encrypt text with p224 curve (default)")
        (@arg PRIME: -p --prime conflicts_with[CRACK] "Encrypt text with Z_{big prime} field")
        (@arg CRACK: -C --crack "Crack small groups")
    ).get_matches();

    if matches.is_present("PRIME") {
        enc_test(zp_encryption());
    } else if matches.is_present("CRACK") {
        crack();
    } else {
        enc_test(ec_encryptor());
    }
}

fn enc_test(enc: PublicEncObject) {
    let (enc, dec) = enc.generate_keys(&mut thread_rng());
    let mut text = Vec::new();
    stdin().lock().read_to_end(&mut text).unwrap();
    let text = String::from_utf8(text).unwrap();
    let cipher = enc.encrypt(&mut thread_rng(), text);
    println!("cipher: {}", cipher.clone());
    let text = dec.decrypt(cipher).unwrap();
    println!("decrypted text: {}", text);
}
