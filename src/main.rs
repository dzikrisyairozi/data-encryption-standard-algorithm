use des_cipher::{des_encrypt, des_decrypt};
use std::io;

fn string_to_u64(s: &str) -> Result<u64, &'static str> {
    let bytes: &[u8] = s.as_bytes();
    if bytes.len() > 8 {
        return Err("Input string too long. Max 8 characters allowed.");
    }

    let mut num: u64 = 0u64;
    for &byte in bytes.iter() {
        num = (num << 8) | byte as u64;
    }
    Ok(num)
}


fn main() {
    println!("Do you want to (1) encrypt or (2) decrypt?");
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: u32 = choice.trim().parse::<u32>().expect("Invalid choice");

    println!("Please enter plaintext or ciphertext:");
    let mut text: String = String::new();
    io::stdin().read_line(&mut text).unwrap();
    let text_num: u64 = string_to_u64(&text.trim()).expect("Invalid input");

    println!("Please enter key:");
    let mut key: String = String::new();
    io::stdin().read_line(&mut key).unwrap();
    let key: u64 = string_to_u64(&text.trim()).expect("Invalid input");

    match choice {
        1 => println!("Encrypted: {:016x}", des_encrypt(text_num, key)),
        2 => println!("Decrypted: {:016x}", des_decrypt(text_num, key)),
        _ => println!("Invalid choice"),
    }
}
