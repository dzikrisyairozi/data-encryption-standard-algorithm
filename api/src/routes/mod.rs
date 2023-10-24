// Import routes from other modules
pub mod encryption;
pub mod decryption;

// Re-export routes so they can be easily used in main.rs
pub use encryption::encrypt;
pub use decryption::decrypt;
