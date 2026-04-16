pub struct VigenereCipher {
    pub key: String,
}

impl VigenereCipher {
    pub fn new(key: &str) -> VigenereCipher {
        VigenereCipher {
            key : key.to_string(),
        }
    }
    //Takes in a letter and shifts it by the shift letter
    //Assumption: Key and the input text is/ will end up as uppercase
    pub fn caesar_shift(letter: char, shift_letter: char) -> char {
        if !letter.is_ascii_alphabetic() {
            return letter;
        }
        let shift = shift_letter.to_ascii_uppercase() as u8 - b'A';
        let base = b'A';
        let shifted = (((letter as u8 + shift - base) % 26) + base) as char;
        return shifted;

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

                // turns c upper case to work with caesar_shift;
                let c_upper = c.to_ascii_uppercase();
                let shifted_char = Self::caesar_shift(c_upper, shift_letter);
                encrypted_message.push(shifted_char);
                //Key index wraps back if it reaches end of key
                key_idx = (key_idx + 1) % key_iter.len();
            } else {
                encrypted_message.push(c);
            }
        }
        encrypted_message
    }
}