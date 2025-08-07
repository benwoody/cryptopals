# Challenge 3: Single-byte XOR cipher

## The Problem

The hex encoded string:

```
1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
```

...has been XOR'd against a **single character**. Find the key, decrypt the message.

## Instructions

You can do this by hand. But don't: write code to do it for you.

**How?** Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.

## Approach

1. **Try all possible keys** (0x00 to 0xFF = 256 possibilities)
2. **XOR the entire message** with each key
3. **Score each result** based on how "English-like" it looks
4. **Pick the highest scoring result**

## Scoring Ideas

- **Letter frequency**: Count common English letters (E, T, A, O, I, N, S, H, R)
- **Printable characters**: Favor results with letters, numbers, spaces, punctuation
- **Avoid control characters**: Penalize results with weird bytes (0x00-0x1F)
- **Word patterns**: Look for common English words

## Expected Output

The decrypted message should be readable English text!
