use std::io;

fn main() {
    println!("Please enter the text to be encrypted:");
    let mut plaintext = String::new();
    io::stdin().read_line(&mut plaintext).expect("Failed to read line");
    let plaintext = plaintext.trim();

    println!("Please enter the encryption key (a number between 1 and 25):");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read line");
    let key: u8 = match key.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for the key, please enter a number between 1 and 25.");
            return;
        }
    };

    let mut ciphertext = String::new();
    for c in plaintext.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let offset = (c as u8 - base + key) % 26;
            ciphertext.push((offset + base) as char);
        } else {
            ciphertext.push(c);
        }
    }

    println!("The encrypted text is: {}", ciphertext);

    let mut decrypted_text = String::new();
    for c in ciphertext.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let offset = (c as u8 - base + 26 - key) % 26;
            decrypted_text.push((offset + base) as char);
        } else {
            decrypted_text.push(c);
        }
    }

    println!("The decrypted text is: {}", decrypted_text);
}
