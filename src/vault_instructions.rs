// Re-export the VaultInstruction from the instruction module
pub use crate::instruction::VaultInstruction;

// Note: The instruction creation functions have been moved to the CLI
// to avoid Borsh serialization issues. These functions are kept for reference
// and can be uncommented once Borsh serialization is properly configured.
