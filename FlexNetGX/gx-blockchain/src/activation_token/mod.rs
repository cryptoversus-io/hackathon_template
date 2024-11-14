//! # Badge and Activation Module
//! mod.rs
//!
//! This module used to manage all activation token state and instructions


pub mod instructions;
pub use instructions::*;

pub mod activation_token_state;
pub use activation_token_state::ActivationTokenState;
