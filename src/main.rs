extern crate cipher_crypt;

use cipher_crypt::{Cipher, Caesar};
use std::io::{self, BufRead};

fn main() {
    
    println!("Enter plaintext.");
    let plaintext = io::stdin().lock().lines().next().unwrap().unwrap();
    println!("Enter offset");
    let offset_str = io::stdin().lock().lines().next().unwrap().unwrap();
    let offset = offset_str.parse::<usize>().unwrap();

    let c = Caesar::new(offset);
    let ciphertext = c.encrypt(&plaintext).unwrap();
    
    println!("Your ciphertext is : {}", &ciphertext);
}
