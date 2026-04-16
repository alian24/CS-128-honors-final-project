use dummy_project::VigenereCipher;

fn main() {
    let key : &str = "ABACADABA";
    let cipher : VigenereCipher = VigenereCipher::new(&key);

    //Shift by the letter 'A' should shift it by 0 letters and return 'A'
    let caesar_shifted = VigenereCipher::caesar_shift('A','A');
    println!("{}", caesar_shifted);
    //shift by Z should shift it by 25 letters, resulting in 'Y'
    let caesar_shifted = VigenereCipher::caesar_shift('A','Z');
    println!("{}", caesar_shifted);

    let original_text = "HELLO WORLD!";
    // should return HFLNO ZOSLD! 
    let encrypted  = cipher.vigenere_encrypt(original_text);
    print!("Orignal Word: {} Encrypted Word: {}", original_text, encrypted);
}