pub fn bytes_to_word(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

pub fn word_to_bytes(word: u16) -> (u8, u8) {
    let high = (word >> 8) as u8;
    let low = word as u8;
    (high, low)
}

pub fn is_bit_set(byte: u8, pos: u8) -> bool {
    let mask = 1 << pos;
    (mask & byte) != 0
}

#[cfg(test)]
mod tests {
    use super::{bytes_to_word, word_to_bytes, is_bit_set};

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

    #[test]
    fn test_is_bit_set() {
        let n = 0xFD;
        assert_eq!(is_bit_set(n, 0), true);
        assert_eq!(is_bit_set(n, 1), false);
        assert_eq!(is_bit_set(n, 2), true);
        assert_eq!(is_bit_set(n, 3), true);
        assert_eq!(is_bit_set(n, 4), true);
        assert_eq!(is_bit_set(n, 5), true);
        assert_eq!(is_bit_set(n, 6), true);
        assert_eq!(is_bit_set(n, 7), true);
    }
}
