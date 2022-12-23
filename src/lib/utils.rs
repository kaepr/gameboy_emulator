
pub fn bytes_to_word(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

pub fn word_to_bytes(word: u16) -> (u8, u8) {
    let high = (word >> 8) as u8;
    let low = word as u8;
    (high, low)
}

#[cfg(test)]
mod tests {
    use super::{bytes_to_word, word_to_bytes};

    #[test]
    fn test_bytes_to_word() {
        let res1 = bytes_to_word(0x10, 0x10);
        assert_eq!(res1, 0x1010);
    }

    #[test]
    fn test_word_to_bytes() {
        let high = 0xab;
        let low = 0xde;
        let word = bytes_to_word(high, low);
        let res1 = word_to_bytes(word);
        assert_eq!(res1, (high, low));
    }
}
