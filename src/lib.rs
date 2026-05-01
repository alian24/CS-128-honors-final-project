pub struct VigenereCipher {
    pub key: String,
}

impl VigenereCipher {
    pub fn new(key: &str) -> Result<VigenereCipher, String> {
        if !Self::is_valid_pokemon(key) {
            return Err("Invalid Pokemon name for the key".to_string());
        }

        Ok(VigenereCipher {
            key : key.to_string(),
        })
    }

    //Takes in a letter and shifts it by the shift letter
    //Assumption: Key and the input text is/ will end up as uppercase
    pub fn caesar_shift(letter: char, shift_letter: char, encrypt: bool) -> char {
        if !letter.is_ascii_alphabetic() {
            return letter;
        }

        let base = if letter.is_ascii_uppercase() { b'A' } else { b'a' };

        let letter_val = letter as u8 - base;
        let shift = shift_letter.to_ascii_uppercase() as u8 - b'A';

        let result = if encrypt {
            (letter_val + shift) % 26
        } else {
            (letter_val + 26 - shift) % 26
        };

        (result + base) as char
    }
    
    pub fn vigenere_encrypt(&self, original_message: &str) -> String {
        let mut encrypted_message: String = Default::default();
        // Filters out non ascii characters and non letters (numbers, spaces, ect.)
        // Then makes the characters uppercase to work with the caesar_shift function
        let key_iter: Vec<char>  = self.key.chars().filter(|c| c.is_ascii_alphabetic()).map(|c| c.to_ascii_uppercase()).collect();

        let mut key_idx = 0;
        // Iterates through the original_message characters running caeser_shift for all the valid chars
        for c in original_message.chars() {
            if c.is_ascii_alphabetic() {
                let shift_letter = key_iter[key_idx];

                let shifted_char = Self::caesar_shift(c, shift_letter, true);
                encrypted_message.push(shifted_char);
                //Key index wraps back if it reaches end of key
                key_idx = (key_idx + 1) % key_iter.len();
            } else {
                encrypted_message.push(c);
            }
        }
        encrypted_message
    }

    pub fn vigenere_decipher(&self, original_message: &str) -> String {
        let mut decipered_message: String = Default::default();
        // Filters out non ascii characters and non letters (numbers, spaces, ect.)
        // Then makes the characters uppercase to work with the caesar_shift function
        let key_iter: Vec<char>  = self.key.chars().filter(|c| c.is_ascii_alphabetic()).map(|c| c.to_ascii_uppercase()).collect();

        let mut key_idx = 0;
        // Iterates through the original_message characters running caeser_shift for all the valid chars
        for c in original_message.chars() {
            if c.is_ascii_alphabetic() {
                let shift_letter = key_iter[key_idx];

                let shifted_char = Self::caesar_shift(c, shift_letter, false);
                decipered_message.push(shifted_char);
                //Key index wraps back if it reaches end of key
                key_idx = (key_idx + 1) % key_iter.len();
            } else {
                decipered_message.push(c);
            }
        }
        decipered_message
    }

    pub fn is_valid_pokemon(name: &str) -> bool {
        let mut input = String::new();
        let mut capitalize_next = true;

        for c in name.trim().to_lowercase().chars() {
            if capitalize_next && c.is_alphabetic() {
                input.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                input.push(c);
                capitalize_next = c == ' ' || c == '-';
            }
        }

        let result = std::panic::catch_unwind(|| {
            pokemon_rs::get_id_by_name(&input, None)
        });

        match result {
            Ok(id) => id != 0,
            Err(_) => false, // panic occurred, treat as invalid Pokémon
        }
    }
}