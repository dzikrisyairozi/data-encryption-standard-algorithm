use std::io;

// Initial and final permutation tables
const IP: [usize; 64] = [
    58, 50, 42, 34, 26, 18, 10, 2,
    60, 52, 44, 36, 28, 20, 12, 4,
    62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8,
    57, 49, 41, 33, 25, 17, 9,  1,
    59, 51, 43, 35, 27, 19, 11, 3,
    61, 53, 45, 37, 29, 21, 13, 5,
    63, 55, 47, 39, 31, 23, 15, 7
];

const FP: [usize; 64] = [
    40, 8, 48, 16, 56, 24, 64, 32,
    39, 7, 47, 15, 55, 23, 63, 31,
    38, 6, 46, 14, 54, 22, 62, 30,
    37, 5, 45, 13, 53, 21, 61, 29,
    36, 4, 44, 12, 52, 20, 60, 28,
    35, 3, 43, 11, 51, 19, 59, 27,
    34, 2, 42, 10, 50, 18, 58, 26,
    33, 1, 41,  9, 49, 17, 57, 25
];


// Expansion function
const E: [usize; 48] = [
    32,  1,  2,  3,  4,  5,
     4,  5,  6,  7,  8,  9,
     8,  9, 10, 11, 12, 13,
    12, 13, 14, 15, 16, 17,
    16, 17, 18, 19, 20, 21,
    20, 21, 22, 23, 24, 25,
    24, 25, 26, 27, 28, 29,
    28, 29, 30, 31, 32,  1
];


// S-boxes
const S: [[[u8; 16]; 4]; 8] = [
    [
        [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
        [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
        [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
        [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
    ],
    [ 
        [ 15,  1,  8, 14,  6, 11,  3,  4,  9, 7,  2, 13, 12, 0,  5, 10],
        [  3, 13,  4,  7, 15,  2,  8, 14, 12, 0,  1, 10,  6, 9, 11,  5],
        [  0, 14,  7, 11, 10,  4, 13,  1,  5, 8, 12,  6,  9, 3,  2, 15],
        [ 13,  8, 10,  1,  3, 15,  4,  2, 11, 6,  7, 12,  0, 5, 14,  9]
    ],
    [ 
        [ 10,  0,  9, 14, 6,  3, 15,  5,  1, 13, 12,  7, 11,  4,  2,  8],
        [ 13,  7,  0,  9, 3,  4,  6, 10,  2,  8,  5, 14, 12, 11, 15,  1],
        [ 13,  6,  4,  9, 8, 15,  3,  0, 11,  1,  2, 12,  5, 10, 14,  7],
        [  1, 10, 13,  0, 6,  9,  8,  7,  4, 15, 14,  3, 11,  5,  2, 12]
    ],
    [ 
        [  7, 13, 14, 3,  0,  6,  9, 10,  1, 2, 8,  5, 11, 12,  4, 15],
        [ 13,  8, 11, 5,  6, 15,  0,  3,  4, 7, 2, 12,  1, 10, 14,  9],
        [ 10,  6,  9, 0, 12, 11,  7, 13, 15, 1, 3, 14,  5,  2,  8,  4],
        [  3, 15,  0, 6, 10,  1, 13,  8,  9, 4, 5, 11, 12,  7,  2, 14]
    ],
    [ 
        [  2, 12,  4,  1,  7, 10, 11,  6,  8,  5,  3, 15, 13, 0, 14,  9],
        [ 14, 11,  2, 12,  4,  7, 13,  1,  5,  0, 15, 10,  3, 9,  8,  6],
        [  4,  2,  1, 11, 10, 13,  7,  8, 15,  9, 12,  5,  6, 3,  0, 14],
        [ 11,  8, 12,  7,  1, 14,  2, 13,  6, 15,  0,  9, 10, 4,  5,  3]
    ],
    [ 
        [ 12,  1, 10, 15,  9,  2,  6,  8,  0, 13,  3,  4, 14,  7,  5, 11],
        [ 10, 15,  4,  2,  7, 12,  9,  5,  6,  1, 13, 14,  0, 11,  3,  8],
        [  9, 14, 15,  5,  2,  8, 12,  3,  7,  0,  4, 10,  1, 13, 11,  6],
        [  4,  3,  2, 12,  9,  5, 15, 10, 11, 14,  1,  7,  6,  0,  8, 13]
    ],
    [ 
        [  4, 11,  2, 14, 15, 0,  8, 13,  3, 12, 9,  7,  5, 10, 6,  1],
        [ 13,  0, 11,  7,  4, 9,  1, 10, 14,  3, 5, 12,  2, 15, 8,  6],
        [  1,  4, 11, 13, 12, 3,  7, 14, 10, 15, 6,  8,  0,  5, 9,  2],
        [  6, 11, 13,  8,  1, 4, 10,  7,  9,  5, 0, 15, 14,  2, 3, 12]
    ],
    [   
        [ 13,  2,  8,  4,  6, 15, 11,  1, 10,  9,  3, 14,  5,  0, 12,  7],
        [  1, 15, 13,  8, 10,  3,  7,  4, 12,  5,  6, 11,  0, 14,  9,  2],
        [  7, 11,  4,  1,  9, 12, 14,  2,  0,  6, 10, 13, 15,  3,  5,  8],
        [  2,  1, 14,  7,  4, 10,  8, 13, 15, 12,  9,  0,  3,  5,  6, 11]
    ]
];


// Permutation function
const P: [usize; 32] = [
    16,  7, 20, 21, 29, 12, 28, 17,
     1, 15, 23, 26,  5, 18, 31, 10,
     2,  8, 24, 14, 32, 27,  3,  9,
    19, 13, 30,  6, 22, 11,  4, 25
];


// Key generation related tables
const PC1: [usize; 56] = [
    57, 49, 41, 33, 25, 17, 9,
    1, 58, 50, 42, 34, 26, 18,
    10, 2, 59, 51, 43, 35, 27,
    19, 11, 3, 60, 52, 44, 36,
    63, 55, 47, 39, 31, 23, 15,
    7, 62, 54, 46, 38, 30, 22,
    14, 6, 61, 53, 45, 37, 29,
    21, 13, 5, 28, 20, 12, 4
];

const PC2: [usize; 48] = [
    14, 17, 11, 24, 1, 5,
    3, 28, 15, 6, 21, 10,
    23, 19, 12, 4, 26, 8,
    16, 7, 27, 20, 13, 2,
    41, 52, 31, 37, 47, 55,
    30, 40, 51, 45, 33, 48,
    44, 49, 39, 56, 34, 53,
    46, 42, 50, 36, 29, 32
];

const LEFT_SHIFTS: [usize; 16] = [
    1, 1, 2, 2, 2, 2, 2, 2,
    1, 2, 2, 2, 2, 2, 2, 1
];


fn initial_permutation(input: u64) -> u64 {
    let mut output: u64 = 0;
    for &i in IP.iter() {
        output <<= 1;
        output |= (input >> (64 - i)) & 1;
    }
    output
}

fn final_permutation(input: u64) -> u64 {
    let mut output: u64 = 0;
    for &i in FP.iter() {
        output <<= 1;
        output |= (input >> (64 - i)) & 1;
    }
    output
}

fn expand(input: u32) -> u64 {
    let mut output: u64 = 0;
    for &i in E.iter() {
        output <<= 1;
        output |= ((input >> (32 - i)) & 1) as u64; // Cast to u64
    }
    output
}


fn s_box(input: u64) -> u32 {
    let mut output: u32 = 0;

    for i in 0..8 {
        let chunk: u8 = ((input >> (42 - 6 * i)) & 0x3F) as u8; // Extract 6 bits
        let row: usize = (((chunk & 0x20) >> 4) | (chunk & 0x01)) as usize; // First and last bit
        let col: usize = ((chunk & 0x1E) >> 1) as usize; // Middle 4 bits

        let s_value: u8 = S[i][row][col];
        output = (output << 4) | s_value as u32;
    }

    output
}

fn permutation(input: u32) -> u32 {
    let mut output: u32 = 0;
    for &i in P.iter() {
        output <<= 1;
        output |= (input >> (32 - i)) & 1;
    }
    output
}

fn key_generation(key: u64) -> [u64; 16] {
    let mut round_keys: [u64; 16] = [0; 16];

    // Apply PC-1 permutation on the key
    let mut permuted_choice_1: u64 = 0;
    for &i in PC1.iter() {
        permuted_choice_1 <<= 1;
        permuted_choice_1 |= (key >> (64 - i)) & 1;
    }

    let mut c = (permuted_choice_1 >> 28) as u32;
    let mut d = (permuted_choice_1 & 0x0FFFFFFF) as u32;

    for round in 0..16 {
        // Left shift for the round
        c = (c << LEFT_SHIFTS[round] | c >> (28 - LEFT_SHIFTS[round])) & 0x0FFFFFFF;
        d = (d << LEFT_SHIFTS[round] | d >> (28 - LEFT_SHIFTS[round])) & 0x0FFFFFFF;

        // Apply PC-2 permutation on concatenated C and D
        let combined = ((c as u64) << 28) | d as u64;
        let mut round_key: u64 = 0;
        for &i in PC2.iter() {
            round_key <<= 1;
            round_key |= (combined >> (56 - i)) & 1;
        }

        round_keys[round] = round_key;
    }

    round_keys
}

fn encrypt(plain: u64, key: u64) -> u64 {
    // Step 1: Initial Permutation
    let mut permuted = initial_permutation(plain);

    // Step 2: Split into left and right halves
    let mut left = (permuted >> 32) as u32;
    let mut right = (permuted & 0xFFFFFFFF) as u32;

    // Step 3: Key generation
    let round_keys = key_generation(key);

    // Step 4: 16 rounds
    for i in 0..16 {
        let prev_right = right;

        // a. Expansion
        let expanded_right = expand(right);

        // b. XOR with round key
        let xor_result = expanded_right ^ round_keys[i];

        // c. S-box processing
        let s_box_result = s_box(xor_result);

        // d. Permutation
        let permuted_s_box = permutation(s_box_result);

        // e. XOR with left half and update halves
        right = left ^ permuted_s_box;
        left = prev_right;
    }

    // Undo the last swap to match DES's "Feistel" structure
    let temp = left;
    left = right;
    right = temp;

    // Step 5: Final Permutation
    let final_result = ((left as u64) << 32) | right as u64;
    final_permutation(final_result)
}

fn decrypt(cipher: u64, key: u64) -> u64 {
    // Step 1: Initial Permutation
    let mut permuted = initial_permutation(cipher);

    // Step 2: Split into left and right halves
    let mut left = (permuted >> 32) as u32;
    let mut right = (permuted & 0xFFFFFFFF) as u32;

    // Step 3: Key generation
    let round_keys = key_generation(key);

    // Step 4: 16 rounds
    for i in (0..16).rev() { // Notice the reverse iteration
        let prev_right = right;

        // a. Expansion
        let expanded_right = expand(right);

        // b. XOR with round key
        let xor_result = expanded_right ^ round_keys[i];

        // c. S-box processing
        let s_box_result = s_box(xor_result);

        // d. Permutation
        let permuted_s_box = permutation(s_box_result);

        // e. XOR with left half and update halves
        right = left ^ permuted_s_box;
        left = prev_right;
    }

    // Undo the last swap to match DES's "Feistel" structure
    let temp = left;
    left = right;
    right = temp;

    // Step 5: Final Permutation
    let final_result = ((left as u64) << 32) | right as u64;
    final_permutation(final_result)
}


fn main() {
    println!("Choose an action:");
    println!("1. Encrypt");
    println!("2. Decrypt");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a number");

    match choice {
        1 => {
            println!("Enter plain text (as a number):");
            let mut plain_text = String::new();
            io::stdin().read_line(&mut plain_text).expect("Failed to read line");
            let plain_text: u64 = plain_text.trim().parse().expect("Please enter a valid number");
            
            println!("Enter key (as a number):");
            let mut key = String::new();
            io::stdin().read_line(&mut key).expect("Failed to read line");
            let key: u64 = key.trim().parse().expect("Please enter a valid number");
            
            let encrypted_text = encrypt(plain_text, key);
            println!("Encrypted text: {}", encrypted_text);
        },
        2 => {
            println!("Enter encrypted text (as a number):");
            let mut encrypted_text = String::new();
            io::stdin().read_line(&mut encrypted_text).expect("Failed to read line");
            let encrypted_text: u64 = encrypted_text.trim().parse().expect("Please enter a valid number");
            
            println!("Enter key (as a number):");
            let mut key = String::new();
            io::stdin().read_line(&mut key).expect("Failed to read line");
            let key: u64 = key.trim().parse().expect("Please enter a valid number");
            
            let decrypted_text = decrypt(encrypted_text, key);
            println!("Decrypted text: {}", decrypted_text);
        },
        _ => {
            println!("Invalid choice");
        },
    }
}
