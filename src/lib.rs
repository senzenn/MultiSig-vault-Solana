use solana_program::{entrypoint, program_error::ProgramError};

// Program ID - this should match your actual program ID
use solana_program::declare_id;
declare_id!("DvMJg65xGz7W7xa1tP6LW2RP4TecJDb5oN2Qcvf7Qc63");

// Export the program ID for use in tests and other modules
pub use crate::ID as PROGRAM_ID;

pub mod instruction;
pub mod defi;
pub mod processor;
pub mod state;
pub mod events;
pub mod protocols;

// Custom error codes for multisig operations
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VaultError {
    // Multisig errors
    InvalidOwner = 0,
    NotEnoughSigners = 1,
    TransactionAlreadyExecuted = 2,
    TransactionAlreadySigned = 3,
    InvalidThreshold = 4,
    MultisigNotInitialized = 5,
    TransactionNotFound = 6,
    InsufficientAuthority = 7,
    InvalidTransactionData = 8,
    UnauthorizedAccess = 9,

    // General vault errors
    InvalidInstruction = 10,
    InvalidAccountData = 11,
    AccountNotRentExempt = 12,
    InvalidAccountOwner = 13,
    ArithmeticOverflow = 14,
    InvalidAmount = 15,
}

impl std::fmt::Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaultError::InvalidOwner => write!(f, "The given owner is not part of this multisig"),
            VaultError::NotEnoughSigners => write!(f, "Not enough owners signed this transaction"),
            VaultError::TransactionAlreadyExecuted => {
                write!(f, "The given transaction has already been executed")
            }
            VaultError::TransactionAlreadySigned => {
                write!(f, "The owner has already signed this transaction")
            }
            VaultError::InvalidThreshold => write!(
                f,
                "Threshold must be less than or equal to the number of owners"
            ),
            VaultError::MultisigNotInitialized => {
                write!(f, "Multisig has not been initialized for this vault")
            }
            VaultError::TransactionNotFound => {
                write!(f, "Transaction with the given ID was not found")
            }
            VaultError::InsufficientAuthority => {
                write!(f, "Insufficient authority to perform this operation")
            }
            VaultError::InvalidTransactionData => write!(f, "Invalid transaction data provided"),
            VaultError::UnauthorizedAccess => write!(f, "Unauthorized access to this operation"),
            VaultError::InvalidInstruction => write!(f, "Invalid instruction data"),
            VaultError::InvalidAccountData => write!(f, "Invalid account data"),
            VaultError::AccountNotRentExempt => write!(f, "Account is not rent exempt"),
            VaultError::InvalidAccountOwner => write!(f, "Invalid account owner"),
            VaultError::ArithmeticOverflow => write!(f, "Arithmetic operation overflow"),
            VaultError::InvalidAmount => write!(f, "Invalid amount specified"),
        }
    }
}

impl From<VaultError> for ProgramError {
    fn from(e: VaultError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// reexport
pub use instruction::*;
pub use defi::*;
pub use processor::*;
pub use state::*;
pub use events::*;
pub use protocols::*;

// emit_event:-> think of it like pusher and logs it with msg macro simple :)
#[macro_export] // this is a way of saying like export emit_event  which means you can use this macro across the project files.
macro_rules! emit_event {
    ($event:ident, $data:expr) => {
        msg!(
            "EVENT: {}",
            serde_json::to_string(&$event)
                .unwrap_or_else(|_| "Failed to serialize event".to_string())
        );
    };
}

// Entry point
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);
