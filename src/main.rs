use dummy_project::VigenereCipher;

fn main() {
    let key : &str = "Pikachu";
    //Stills errors if the key is not a Pokemon, fix later, however it works if the key is a pokemon.
    //The errors seems come from the crate
    //Goal: print the error on the gui instead of crashing
    let cipher : VigenereCipher = match VigenereCipher::new(&key) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to create cipher: {}", e);
            return;
        }
    };

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

    let is_poke =  VigenereCipher::is_valid_pokemon("Pikachu");
    println!("Pikachu is a pokemon: {}", is_poke);
}