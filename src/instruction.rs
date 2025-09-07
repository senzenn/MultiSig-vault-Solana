use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};
use crate::state::{GovernanceInstruction, VoteType};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum VaultInstruction {
    // why we need this-> every transaction to your program will specify which variant to execute
    // sol -> serialze instructions -> then send it to your program
    Initialize {
        bump: u8, // initial vault creates PDA
    },
    Deposite {
        amount: u64, // V
    },
    Withdraw {
        amount: u64, // V and Security
    },
    Transfer {
        recipient: Pubkey,
        amount: u64, // Transfer SOL directly to another account
    },
    InitializeMultiSig {
        // multi sig
        owners: Vec<Pubkey>,
        threshold: u64, // min sig
        nonce: u8,      // for PDA derivation
    }, // it is required for creating 3-5 signature for large withdrawals

    CreateMultiSigTransaction {
        // Create a new multisig transaction
        program_id: Pubkey,
        accounts: Vec<crate::state::TransactionAccount>,
        data: Vec<u8>,
    },
    ApproveMultiSigTransaction {
        transaction_id: u64,
    },
    ExecuteMultiSigTransaction {
        transaction_id: u64,
    },
    SetMultiSigOwners {
        owners: Vec<Pubkey>,
    },
    ChangeMultiSigThreshold {
        threshold: u64,
    },

    CreateProposal {
        // it Approves a Pending transaction for execution
        instruction_data: Vec<u8>,
    },
    ApproveProposal {
        proposal_id: u64,
    },
    ExecuteProposal {
        proposal_id: u64,
    },
    RejectProposal {
        proposal_id: u64,
    },

    PauseVault,   // emergency pause
    UnpauseVault, // resume operations
    EmergencyWithdraw {
        token_mint: Pubkey,
        amount: u64,
    },
    AddSupportedToken {
        mint: Pubkey,
        bump: u8,
    },
    DepositMultiToken {
        mint: Pubkey,
        amount: u64,
    },
    CreateTimeLock {
        beneficiary: Pubkey,
        amount: u64,
        duration: i64,
        cliff_duration: Option<i64>,
        is_linear: bool,
    },
    ClaimTimeLock {
        time_lock_index: usize,
    },
    CancelTimeLock {
        time_lock_index: usize,
    },
    SetYieldStrategy {
        token_mint: Pubkey,
        strategy_program: Pubkey,
    },
    HarvestYield {
        token_mint: Pubkey,
    },
    CompoundYield {
        token_mint: Pubkey,
    },
    JupiterSwap {
        input_mint: Pubkey,
        output_mint: Pubkey,
        amount: u64,
    },

    JupiterRoute {
        input_mint: Pubkey,
        output_mint: Pubkey,
        amount: u64,
        route: Vec<u8>,
    },
    CollectFees,
    TransferAuthority {
        new_authority: Pubkey,
    },
    UpdateEmergencyAdmin {
        new_admin: Pubkey,
    },
    InitializeGovernance {
        voting_token_mint: Pubkey,
        quorum_threshold: u16,
        proposal_threshold: u64,
        voting_period: i64,
        time_lock_delay: i64,
        execution_threshold: u16,
    },
    CreateGovernanceProposal {
        title: String,
        description: String,
        instructions: Vec<Vec<u8>>,
    },
    CastVote {
        proposal_id: u64,
        vote_type: crate::state::VoteType,
    },
    QueueProposal {
        proposal_id: u64,
    },
    ExecuteGovernanceProposal {
        proposal_id: u64,
    },
    UpdateGovernanceConfig {
        quorum_threshold: u16,
        proposal_threshold: u64,
        voting_period: i64,
        time_lock_delay: i64,
        execution_threshold: u16,
    },
}

impl Default for VaultInstruction {
    fn default() -> Self {
        VaultInstruction::Initialize { bump: 0 }
    }
}
