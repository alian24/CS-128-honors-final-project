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
}