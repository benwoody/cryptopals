//! Challenge 1: Convert hex to base64
//! 
//! The string:
//! 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
//! 
//! Should produce:
//! SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t

use crate::utils::encoding::hex_to_base64;

/// Solution for Challenge 1
pub fn solve_challenge1() -> Result<(), Box<dyn std::error::Error>> {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    
    let result = hex_to_base64(input)?;
    
    println!("Challenge 1: Convert hex to base64");
    println!("Input (hex): {}", input);
    println!("Output (base64): {}", result);
    println!("Expected: {}", expected);
    println!("Match: {}", result == expected);
    
    Ok(())
}