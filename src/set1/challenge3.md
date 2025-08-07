# Challenge 3: Single-byte XOR cipher

## The Problem

The hex encoded string:

```
1b3GG7373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
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

## The Work (stream)

```
1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
```

I've done cryptoquotes before... look for doubles? `ee` `oo` `ll` etc...

There's a few doubles. At the start we have: `1b 37 37 33 31 ...`

Try `ee` which is 0x65:

`37 ^ 65` 

```
0x37 = 00110111 
XOR  = 01010010 = 0x52
----
e    = 01100101

1b ^ 52 = 0x49 = I 
37 ^ 52 = 0x65 = e  
37 ^ 52 = 0x65 = e 
33 ^ 52 = 0x61 = a
```
Nope. Let's try `o`:

```
0x37 = 00110111 
XOR  = 01011000 = 0x58
----
o    = 01101111

1b ^ 58 = 0x43 = C 
37 ^ 58 = 0x6F = o
37 ^ 58 = 0x6F = o
33 ^ 58 = 0x6B = k
```

YOOO OK

```
31 ^ 58 = 0x69 = i
36 ^ 58 = 0x6E = n
3f ^ 58 = 0x67 = g
78 ^ 58 = 0x20 = (space)
15 ^ 58 = 0x4D = M
1b ^ 58 = 0x43 = C
7f ^ 58 = 0x27 = '
2b ^ 58 = 0x73 = s
78 ^ 58 = 0x20 = (space)
34 ^ 58 = 0x6C = l
31 ^ 58 = 0x69 = i
33 ^ 58 = 0x6B = k
3d ^ 58 = 0x65 = e
78 ^ 58 = 0x20 = (space)
39 ^ 58 = 0x61 = a
78 ^ 58 = 0x20 = (space)
28 ^ 58 = 0x70 = p
37 ^ 58 = 0x6F = o
2d ^ 58 = 0x75 = u
36 ^ 58 = 0x6E = n
3c ^ 58 = 0x64 = d
78 ^ 58 = 0x20 = (space)
37 ^ 58 = 0x6F = o
3e ^ 58 = 0x66 = f
78 ^ 58 = 0x20 = (space)
3a ^ 58 = 0x62 = b
39 ^ 58 = 0x61 = a
3b ^ 58 = 0x63 = c
37 ^ 58 = 0x6F = o
36 ^ 58 = 0x6E = n
```

I'd like to solve the puzzle, Alex.

Cooking MC's like a pound of bacon