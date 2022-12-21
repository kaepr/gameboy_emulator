pub fn bytes_to_word(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

#[cfg(test)]
mod tests {
    use super::bytes_to_word;

    #[test]
    fn test_bytes_to_word() {
        let res1 = bytes_to_word(0x10, 0x10);
        assert_eq!(res1, 0x1010);
    }
}
