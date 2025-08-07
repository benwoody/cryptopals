//! XOR operation utilities
//! 
//! Common XOR operations used throughout the Cryptopals challenges

/// XOR two byte arrays of equal length
pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len(), "Byte arrays must be of equal length");
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

/// XOR a byte array with a single byte (repeating key)
pub fn xor_single_byte(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|&b| b ^ key).collect()
}

/// XOR a byte array with a repeating key
pub fn xor_repeating_key(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .zip(key.iter().cycle())
        .map(|(&d, &k)| d ^ k)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_bytes() {
        let a = vec![0x1c, 0x01, 0x11];
        let b = vec![0x68, 0x69, 0x74];
        let expected = vec![0x74, 0x68, 0x65];
        
        assert_eq!(xor_bytes(&a, &b), expected);
    }

    #[test]
    fn test_xor_single_byte() {
        let data = vec![0x1b, 0x37, 0x37];
        let key = 0x58;
        let expected = vec![0x43, 0x6f, 0x6f];
        
        assert_eq!(xor_single_byte(&data, key), expected);
    }
}
