pub enum VaultInstruction {
    Initialize {
        bump: u8,
    },
    Deposite {
        amount: u64,
    },
    Withdraw {
        amount: u64,
    },
    InitializeMultiSig {
        authorities: Vec<Pubkey>,
        threshold: u8,
        bump: u8,
    },

    CreateProposal {
        instruction: Box<VaultInstruction>,
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
    PauseVault,
    unPauseVault,
    EmergencyWithdraw {
        token_mint: Pubkey,
        amount: u64,
    },
    AddSupportedToken {
        mint: Pubkey,
        bump: u8,
    },
    DepositeMultiToken {
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

    // Jup
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
        quorm_threshold: u16,
        proposal_threshold: u64,
        voting_period: i64,
        time_lock_delay: i64,
        execution_threshold: u16,
    },
    CreateGovernanceProposal {
        title: String,
        description: String,
        instructions: Vec<GovernanceInstruction>,
    },
    CasteVote {
        proposal_id: u64,
        vote_type: VoteType,
    },
    QueueProposal {
        proposal_id: u64,
    },
    ExecuteGovernanceProposal {
        proposal_id: u64,
    },

    UpdateGovernanceConfig {
        quorm_threshold: u16,
        proposal_threshold: u64,
        voting_period: i64,
        time_lock_delay: i64,
        execution_threshold: u16,
    },
}
