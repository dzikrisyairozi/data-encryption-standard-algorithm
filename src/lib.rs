pub mod utils;
pub mod constants;

use utils::permutations::*;
use utils::sboxes::*;
use utils::key_gen::*;
use constants::shift::*;

fn initial_permutation(input: u64) -> u64 {
    let mut output: u64 = 0;  // Initialize the output variable to store the permuted result.
    for &i in IP.iter() {     // Iterate through the elements of the 'IP' array.
        output <<= 1;         // Left shift the 'output' by one bit position.
        output |= (input >> (64 - i)) & 1; // Extract and insert a specific bit from 'input' to 'output'.
    }
    output                   // Return the permuted 'output'.
}

fn final_permutation(input: u64) -> u64 {
    let mut output: u64 = 0;  // Initialize the output variable to store the permuted result.
    for &i in FP.iter() {     // Iterate through the elements of the 'FP' array.
        output <<= 1;         // Left shift the 'output' by one bit position.
        output |= (input >> (64 - i)) & 1; // Extract and insert a specific bit from 'input' to 'output'.
    }
    output                     // Return the permuted 'output'.
}

fn expand(input: u32) -> u64 {
    let mut output: u64 = 0;  // Initialize the output variable to store the expanded result.
    for &i in E.iter() {       // Iterate through the elements of the 'E' array.
        output <<= 1;         // Left shift the 'output' by one bit position.
        output |= ((input >> (32 - i)) & 1) as u64; // Extract and insert a specific bit from 'input' to 'output', casting to u64.
    }
    output                   // Return the expanded 'output'.
}

fn s_box(input: u64) -> u32 {
    let mut output: u32 = 0;  // Initialize the output variable to store the S-box substituted result.

    for i in 0..8 {           // Iterate through 8 groups of 6 bits in the 'input'.
        let chunk: u8 = ((input >> (42 - 6 * i)) & 0x3F) as u8; // Extract 6 bits from 'input'.
        let row: usize = (((chunk & 0x20) >> 4) | (chunk & 0x01)) as usize; // Calculate row index for S-box lookup.
        let col: usize = ((chunk & 0x1E) >> 1) as usize; // Calculate column index for S-box lookup.

        let s_value: u8 = S[i][row][col]; // Look up the S-box value.
        output = (output << 4) | s_value as u32; // Append the S-box value to the 'output'.
    }

    output                   // Return the S-box substituted 'output'.
}

fn permutation(input: u32) -> u32 {
    let mut output: u32 = 0;  // Initialize the output variable to store the permuted result.
    for &i in P.iter() {      // Iterate through the elements of the 'P' array.
        output <<= 1;         // Left shift the 'output' by one bit position.
        output |= (input >> (32 - i)) & 1; // Extract and insert a specific bit from 'input' to 'output'.
    }
    output                   // Return the permuted 'output'.
}

fn key_generation(key: u64) -> [u64; 16] {
    let mut round_keys: [u64; 16] = [0; 16]; // Initialize an array to store round keys.

    // Apply PC-1 permutation on the key
    let mut permuted_choice_1: u64 = 0;  // Initialize a variable to store the PC-1 permuted key.
    for &i in PC1.iter() {               // Iterate through the elements of the 'PC1' array.
        permuted_choice_1 <<= 1;         // Left shift the permuted key by one bit position.
        permuted_choice_1 |= (key >> (64 - i)) & 1; // Extract and insert a specific bit from 'key' to 'permuted_choice_1'.
    }

    let mut c: u32 = (permuted_choice_1 >> 28) as u32; // Initialize the left half of the key.
    let mut d: u32 = (permuted_choice_1 & 0x0FFFFFFF) as u32; // Initialize the right half of the key.

    for round in 0..16 {  // Iterate through the 16 rounds of key generation.
        // Left shift for the round
        c = (c << LEFT_SHIFTS[round] | c >> (28 - LEFT_SHIFTS[round])) & 0x0FFFFFFF; // Left shift and mask.
        d = (d << LEFT_SHIFTS[round] | d >> (28 - LEFT_SHIFTS[round])) & 0x0FFFFFFF; // Left shift and mask.

        // Apply PC-2 permutation on concatenated C and D
        let combined: u64 = ((c as u64) << 28) | d as u64; // Concatenate C and D.
        let mut round_key: u64 = 0; // Initialize a variable to store the round key.
        for &i in PC2.iter() { // Iterate through the elements of the 'PC2' array.
            round_key <<= 1;  // Left shift the round key by one bit position.
            round_key |= (combined >> (56 - i)) & 1; // Extract and insert a specific bit from 'combined' to 'round_key'.
        }

        round_keys[round] = round_key; // Store the round key in the array.
    }

    round_keys // Return the array of round keys.
}


pub fn des_encrypt(plain: u64, key: u64) -> u64 {
    // Step 1: Initial Permutation
    let permuted: u64 = initial_permutation(plain);

    // Step 2: Split into left and right halves
    let mut left: u32 = (permuted >> 32) as u32;
    let mut right: u32 = (permuted & 0xFFFFFFFF) as u32;

    // Step 3: Key generation
    let round_keys: [u64; 16] = key_generation(key);

    // Step 4: 16 rounds
    for i in 0..16 {
        let prev_right: u32 = right;

        // a. Expansion
        let expanded_right: u64 = expand(right);

        // b. XOR with round key
        let xor_result: u64 = expanded_right ^ round_keys[i];

        // c. S-box processing
        let s_box_result: u32 = s_box(xor_result);

        // d. Permutation
        let permuted_s_box: u32 = permutation(s_box_result);

        // e. XOR with left half and update halves
        right = left ^ permuted_s_box;
        left = prev_right;
    }

    // Undo the last swap to match DES's "Feistel" structure
    let temp: u32 = left;
    left = right;
    right = temp;

    // Step 5: Final Permutation
    let final_result: u64 = ((left as u64) << 32) | right as u64;
    final_permutation(final_result)
}

pub fn des_decrypt(cipher: u64, key: u64) -> u64 {
    // Step 1: Initial Permutation
    let permuted: u64 = initial_permutation(cipher);

    // Step 2: Split into left and right halves
    let mut left: u32 = (permuted >> 32) as u32;
    let mut right: u32 = (permuted & 0xFFFFFFFF) as u32;

    // Step 3: Key generation
    let round_keys: [u64; 16] = key_generation(key);

    // Step 4: 16 rounds
    for i in (0..16).rev() { // Notice the reverse iteration
        let prev_right: u32 = right;

        // a. Expansion
        let expanded_right: u64 = expand(right);

        // b. XOR with round key
        let xor_result: u64 = expanded_right ^ round_keys[i];

        // c. S-box processing
        let s_box_result: u32 = s_box(xor_result);

        // d. Permutation
        let permuted_s_box: u32 = permutation(s_box_result);

        // e. XOR with left half and update halves
        right = left ^ permuted_s_box;
        left = prev_right;
    }

    // Undo the last swap to match DES's "Feistel" structure
    let temp: u32 = left;
    left = right;
    right = temp;

    // Step 5: Final Permutation
    let final_result: u64 = ((left as u64) << 32) | right as u64;
    final_permutation(final_result)
}