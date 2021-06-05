use std::{io::{stdin, Read}, sync::mpsc, thread};

use attack::Attack;
use clap::clap_app;
use dlies::zp_encryption;
use rand::thread_rng;

use crate::{
    attack::{CCA, CPA},
    ecges::ec_encryptor,
    encryption::extensions::PublicEncObject,
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
        (@arg CURVE: -c --curve conflicts_with[PRIME CPA CCA] "Encrypt text with p224 curve (default)")
        (@arg PRIME: -p --prime conflicts_with[CPA CCA] "Encrypt text with Z_{big prime} field")
        (@arg CPA: --cpa conflicts_with[CCA] "Try to break both encoders using CPA [TBD]")
        (@arg CCA: --cca "Break both encoders using CCA [TBD]")
    ).get_matches();

    if matches.is_present("PRIME") {
        enc_test(zp_encryption());
    } else if matches.is_present("CPA") {
        attack::<CPA>();
    } else if matches.is_present("CCA") {
        attack::<CCA>();
    } else {
        enc_test(ec_encryptor());
    }
}

fn attack<A: Attack>() {
    let (tx, rx) = mpsc::channel::<String>();
    let tx1 = tx.clone();
    let curve = thread::spawn(move || A::attack_on_curve(tx1));
    let prime = thread::spawn(move || A::attack_on_prime(tx));
    for msg in rx {
        println!("{}", msg);
    }
    prime.join().unwrap();
    curve.join().unwrap();
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
