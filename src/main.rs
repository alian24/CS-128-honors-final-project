use dummy_project::VigenereCipher;

fn main() {
    let key : &str = "ABACADABA";
    let cipher : VigenereCipher = VigenereCipher::new(&key);

    //Shift by the letter 'A' should shift it by 0 letters and return 'A'
    let caesar_shifted = VigenereCipher::caesar_shift('A','A', true);
    println!("Original letter: A, Caesar shifted letter: {}", caesar_shifted);
    //shift by Z should shift it by 25 letters, resulting in 'Z'
    let caesar_shifted = VigenereCipher::caesar_shift('A','Z', true);
    println!("Original letter: A, Caesar shifted letter: {}", caesar_shifted);

    let original_text = "Hello world!";
    // should return HFLNO ZOSLD! 
    let encrypted  = cipher.vigenere_encrypt(original_text);
    println!("Orignal Word: {} Encrypted Word: {}", original_text, encrypted);
    // should return HELLO WORLD!
    let decipered = cipher.vigenere_decipher(encrypted.as_str());
    println!("Orignal Word: {} Encrypted Word: {}", encrypted, decipered);
}