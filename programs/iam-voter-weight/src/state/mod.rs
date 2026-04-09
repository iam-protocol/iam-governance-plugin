pub mod registrar;
pub use registrar::*;

// Wrap the addin API VoterWeightRecord for Anchor compatibility
vote_weight_record!(crate::ID);
