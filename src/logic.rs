//! This library contains the game's logic.
//! - `game_logic` : contains the rules and definitions for playing and biddings phases.
//! - `server_logic` : contains the logic used by the server to update the state

#![allow(dead_code)]

pub mod client_logic;
pub mod playing;
pub mod bidding;
pub mod round;
pub mod party;
