pub struct Opts {
    pub show_debug_info: bool,
    pub show_serial_output: bool,
}

impl Opts {
    pub fn new(debug: bool, serial: bool) -> Self {
        Opts {
            show_debug_info: debug,
            show_serial_output: serial,
        }
    }
}

pub fn bytes_to_word(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

pub fn word_to_bytes(word: u16) -> (u8, u8) {
    let high = (word >> 8) as u8;
    let low = word as u8;
    (high, low)
}

pub fn is_bit_set(byte: u8, pos: usize) -> bool {
    let mask = 1 << pos;
    (byte & mask) != 0
}

pub fn is_bit_set_16(byte: u16, pos: usize) -> bool {
    let mask = 1 << pos;
    (byte & mask) != 0
}

// pub fn is_bit_set(byte: u8, pos: u8) -> bool {
//     let mask = 1 << pos;
//     (mask & byte) != 0
// }

pub fn reset_bit(byte: u8, pos: u8) -> u8 {
    let mask = 1 << pos;
    byte & !mask
}

pub fn set_bit(byte: u8, pos: u8) -> u8 {
    let mask = 1 << pos;
    byte | mask
}

pub fn swap_nibbles(byte: u8) -> u8 {
    byte.rotate_right(4)
}

pub fn le_bytes_to_word(low: u8, high: u8) -> u16 {
    u16::from_le_bytes([low, high])
}

pub fn is_half_carry_inc8(a: u8, b: u8) -> bool {
    (((a & 0x0F) + (b & 0x0F)) & 0x10) == 0x10
}

pub fn rotate_left_helper(byte: u8, prev_carry: bool, through_carry: bool) -> (u8, bool) {
    if through_carry {
        let carry = is_bit_set(byte, 7);
        let mut res = byte.rotate_left(1);
        res = reset_bit(res, 0);
        if prev_carry {
            res = set_bit(res, 0);
        }
        (res, carry)
    } else {
        let carry = is_bit_set(byte, 7);
        let res = byte.rotate_left(1);
        (res, carry)
    }
}

pub fn rotate_right_helper(byte: u8, prev_carry: bool, through_carry: bool) -> (u8, bool) {
    if through_carry {
        let carry = is_bit_set(byte, 0);
        let mut res = byte.rotate_right(1);
        res = reset_bit(res, 7);
        if prev_carry {
            res = set_bit(res, 7);
        }
        (res, carry)
    } else {
        let carry = is_bit_set(byte, 0);
        let res = byte.rotate_right(1);
        (res, carry)
    }
}

pub trait HalfCarryCheck {
    type Item;

    fn half_carry_add(&self, other: Self::Item) -> bool;
    fn half_carry_sub(&self, other: Self::Item) -> bool;
}

impl HalfCarryCheck for u8 {
    type Item = u8;

    fn half_carry_add(&self, other: Self::Item) -> bool {
        (((self & 0x0F) + (other & 0x0F)) & 0xF0) > 0x0F
    }

    fn half_carry_sub(&self, other: Self::Item) -> bool {
        (self & 0x0F) < (other & 0x0F)
    }
}

impl HalfCarryCheck for u16 {
    type Item = u16;

    fn half_carry_add(&self, other: Self::Item) -> bool {
        ((self & 0x0FFF) + (other & 0x0FFF)) > 0x0FFF
    }

    fn half_carry_sub(&self, other: Self::Item) -> bool {
        (self & 0x0F) < (other & 0x0F)
    }
}

#[cfg(test)]
mod tests {
    use super::{bytes_to_word, is_bit_set, is_half_carry_inc8, word_to_bytes};

    #[test]
    fn test_half_carry() {
        assert_eq!(true, is_half_carry_inc8(10, 12));
        assert_eq!(false, is_half_carry_inc8(5, 4));
    }

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
