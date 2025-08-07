//! Encoding/Decoding utilities
//! 
//! Common encoding operations used throughout the Cryptopals challenges

use hex;
use base64::{Engine as _, engine::general_purpose};

/// Convert a hex string to base64
pub fn hex_to_base64(hex_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Decode hex to bytes
    let bytes = hex::decode(hex_str)?;
    
    // Encode bytes to base64
    let base64_str = general_purpose::STANDARD.encode(&bytes);
    
    Ok(base64_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        
        let result = hex_to_base64(input).unwrap();
        assert_eq!(result, expected);
    }
}
