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

pub fn rotate_left_helper(byte: u8, prev_carry: bool, through_carry: bool) -> (u8, bool) {
    if through_carry {
        let carry = byte.is_bit_set(7);
        let mut res = byte.rotate_left(1);
        res = reset_bit(res, 0);
        if prev_carry {
            res = set_bit(res, 0);
        }
        (res, carry)
    } else {
        let carry = byte.is_bit_set(7);
        let res = byte.rotate_left(1);
        (res, carry)
    }
}

pub fn rotate_right_helper(byte: u8, prev_carry: bool, through_carry: bool) -> (u8, bool) {
    if through_carry {
        let carry = byte.is_bit_set(0);
        let mut res = byte.rotate_right(1);
        res = reset_bit(res, 7);
        if prev_carry {
            res = set_bit(res, 7);
        }
        (res, carry)
    } else {
        let carry = byte.is_bit_set(0);
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

pub trait BitPosCheck {
    fn is_bit_set(&self, position: usize) -> bool;
}

impl BitPosCheck for u8 {
    fn is_bit_set(&self, position: usize) -> bool {
        (self & (1 << position)) != 0
    }
}

impl BitPosCheck for u16 {
    fn is_bit_set(&self, position: usize) -> bool {
        (self & (1 << position)) != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::BitPosCheck;

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

    #[test]
    fn test_is_bit_set() {
        let n: u8 = 0xFD;
        assert!(n.is_bit_set(0));
        assert!(n.is_bit_set(1) == false);
        assert!(n.is_bit_set(2));
        assert!(n.is_bit_set(3));
        assert!(n.is_bit_set(4));
        assert!(n.is_bit_set(5));
        assert!(n.is_bit_set(6));
        assert!(n.is_bit_set(7));
    }
}
