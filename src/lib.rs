pub mod utils;
pub mod constants;

use utils::permutations::*;
use utils::sboxes::*;
use utils::key_gen::*;
use constants::shift::*;

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

    let mut c: u32 = (permuted_choice_1 >> 28) as u32;
    let mut d: u32 = (permuted_choice_1 & 0x0FFFFFFF) as u32;

    for round in 0..16 {
        // Left shift for the round
        c = (c << LEFT_SHIFTS[round] | c >> (28 - LEFT_SHIFTS[round])) & 0x0FFFFFFF;
        d = (d << LEFT_SHIFTS[round] | d >> (28 - LEFT_SHIFTS[round])) & 0x0FFFFFFF;

        // Apply PC-2 permutation on concatenated C and D
        let combined: u64 = ((c as u64) << 28) | d as u64;
        let mut round_key: u64 = 0;
        for &i in PC2.iter() {
            round_key <<= 1;
            round_key |= (combined >> (56 - i)) & 1;
        }

        round_keys[round] = round_key;
    }

    round_keys
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