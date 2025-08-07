//! Utility functions for cryptographic operations
//! 
//! This module contains common functions that are used across
//! multiple challenges, such as XOR operations, frequency analysis,
//! and other cryptographic utilities.

pub mod xor;
pub mod encoding;

pub use xor::*;
pub use encoding::*;

// You can add more utility modules here as you progress through the challenges
// pub mod analysis;
// pub mod padding;
// pub mod aes;
