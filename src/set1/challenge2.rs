//! Challenge 2: Fixed XOR
// Write a function that takes two equal-length buffers and produces their XOR combination.
//
// If your function works properly, then when you feed it the string:
//
// 1c0111001f010100061a024b53535009181c
// ... after hex decoding, and when XOR'd against:
//
// 686974207468652062756c6c277320657965
// ... should produce:
//
// 746865206b696420646f6e277420706c6179

use crate::utils::xor::xor_bytes;

pub fn solve_challenge2() -> Result<(), Box<dyn std::error::Error>> {
    let input1 = hex::decode("1c0111001f010100061a024b53535009181c")?;
    let input2 = hex::decode("686974207468652062756c6c277320657965")?;
    let expected = hex::decode("746865206b696420646f6e277420706c6179")?;

    let result = xor_bytes(&input1, &input2);
    assert_eq!(result, expected);

    Ok(())
}