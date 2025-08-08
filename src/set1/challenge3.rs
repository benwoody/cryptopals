// Challenge 3: Single-byte XOR cipher
// The hex encoded string:
// 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
// ... has been XOR'd against a single character. Find the key, decrypt the message.

// You can do this by hand. But don't: write code to do it for you.

// How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.

use crate::utils::xor::xor_single_byte;
use hex;

/// Score text based on how "English-like" it appears
/// Higher scores indicate more likely English text
fn score_english_text(text: &[u8]) -> f32 {
    // English letter frequency (approximate percentages)
    let freq_map: [f32; 26] = [
        8.12, 1.49, 2.78, 4.25, 12.02, 2.23, 2.02, 6.09, 6.97, 0.15, 0.77, 4.03, 2.41,
        6.75, 7.51, 1.93, 0.10, 5.99, 6.33, 9.06, 2.76, 0.98, 2.36, 0.15, 1.97, 0.07
    ];
    
    let mut score = 0.0;
    let mut total_letters = 0;
    
    for &byte in text {
        match byte {
            // Common printable characters get positive scores
            b' ' => score += 13.0,  // Space is very common
            b'e' | b'E' => score += freq_map[4],   // E is most frequent
            b't' | b'T' => score += freq_map[19],  // T is second most frequent  
            b'a' | b'A' => score += freq_map[0],   // A
            b'o' | b'O' => score += freq_map[14],  // O
            b'i' | b'I' => score += freq_map[8],   // I
            b'n' | b'N' => score += freq_map[13],  // N
            b's' | b'S' => score += freq_map[18],  // S
            b'h' | b'H' => score += freq_map[7],   // H
            b'r' | b'R' => score += freq_map[17],  // R
            
            // Other letters get smaller positive scores
            b'A'..=b'Z' | b'a'..=b'z' => {
                score += 1.0;
                total_letters += 1;
            },
            
            // Numbers and basic punctuation are okay
            b'0'..=b'9' | b'.' | b',' | b'!' | b'?' | b'\'' | b'"' => score += 0.5,
            
            // Newlines and tabs are acceptable
            b'\n' | b'\t' => score += 0.1,
            
            // Control characters and high ASCII are bad
            0..=31 | 127..=255 => score -= 10.0,
            
            // Other printable characters are neutral
            _ => score += 0.0,
        }
    }
    
    // Bonus for reasonable letter-to-total ratio
    if total_letters > 0 && text.len() > 0 {
        let letter_ratio = total_letters as f32 / text.len() as f32;
        if letter_ratio > 0.6 && letter_ratio < 0.9 {
            score += 10.0;
        }
    }
    
    score
}

/// Try to break single-byte XOR cipher by testing all possible keys
fn break_single_byte_xor(ciphertext: &[u8]) -> Option<(u8, Vec<u8>, f32)> {
    let mut best_score = f32::MIN;
    let mut best_key = 0u8;
    let mut best_plaintext = Vec::new();
    
    // Try all possible single-byte keys (0x00 to 0xFF)
    for key in 0..=255u8 {
        let plaintext = xor_single_byte(ciphertext, key);
        let score = score_english_text(&plaintext);
        
        if score > best_score {
            best_score = score;
            best_key = key;
            best_plaintext = plaintext;
        }
    }
    
    if best_score > 0.0 {
        Some((best_key, best_plaintext, best_score))
    } else {
        None
    }
}

/// Solution for Challenge 3
pub fn solve_challenge3() -> Result<(), Box<dyn std::error::Error>> {
    let hex_input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    
    // Decode hex to bytes
    let ciphertext = hex::decode(hex_input)?;
    
    println!("Challenge 3: Single-byte XOR cipher");
    println!("Ciphertext (hex): {}", hex_input);
    println!();
    
    // Try to break the cipher
    match break_single_byte_xor(&ciphertext) {
        Some((key, plaintext, score)) => {
            // Convert plaintext to string for display
            let plaintext_str = String::from_utf8_lossy(&plaintext);
            
            println!("🎉 Cipher broken!");
            println!("Key found: 0x{:02x} ('{}')", key, key as char);
            println!("Score: {:.2}", score);
            println!("Decrypted message: \"{}\"", plaintext_str);
            
            // Verify by re-encrypting
            let re_encrypted = xor_single_byte(&plaintext, key);
            let verification = hex::encode(&re_encrypted);
            println!();
            println!("Verification:");
            println!("Re-encrypted: {}", verification);
            println!("Original:     {}", hex_input);
            println!("Match: {}", verification == hex_input);
        },
        None => {
            println!("❌ Could not break the cipher");
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_english_text() {
        let english_text = b"Hello world";
        let gibberish = b"\x01\x02\x03\x04";
        
        let english_score = score_english_text(english_text);
        let gibberish_score = score_english_text(gibberish);
        
        assert!(english_score > gibberish_score);
        assert!(english_score > 0.0);
    }

    #[test]
    fn test_challenge3_solution() {
        let hex_input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let ciphertext = hex::decode(hex_input).unwrap();
        
        let result = break_single_byte_xor(&ciphertext);
        assert!(result.is_some());
        
        let (key, plaintext, _score) = result.unwrap();
        let plaintext_str = String::from_utf8_lossy(&plaintext);
        
        // The actual solution should contain readable English
        assert!(plaintext_str.contains("Cooking")); // Spoiler: it's about cooking
        assert_eq!(key, 0x58); // The key should be 'X'
    }
}
