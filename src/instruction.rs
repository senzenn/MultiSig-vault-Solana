pub enum VaultInstruction {
// why  we need this->  every transaction to your program will specify which variant to execute
// sol -> serialze instructions -> then send it to your program
    Initialize {
        bump: u8, // initial vault creates  PDA
    },
    Deposite {
        amount: u64, // V
    },
    Withdraw {
        amount: u64, // V and Security
    },
    InitializeMultiSig { // multi sig
        authorities: Vec<Pubkey>,
        threshold: u8, // min sig
        bump: u8, // pda creation
    }, // it is required for creating 3-5 signature for large withdrawals

    CreateProposal { // it Approves a Pending transaction for execution
        instruction: Box<VaultInstruction>,
    },
    ApproveProposal {
        proposal_id: u64,
    },
    ExecuteProposal {
        proposal_id: u64,
    },
    RejectProposal { // halt exectiution of alll programs
        proposal_id: u64,
    },
    PauseVault,// temporary stop all transactions
    unPauseVault,  // toggle pause
    EmergencyWithdraw {  // allow admin  withdraw as a emergency
        token_mint: Pubkey,
        amount: u64,
    }, // only admin can do this
    AddSupportedToken { // add new token to the vault
        mint: Pubkey,
        bump: u8,
    },
    DepositeMultiToken {  // non-SOL token
        mint: Pubkey,
        amount: u64,
    },
    CreateTimeLock { // kind of like stream flow
        beneficiary: Pubkey,
        amount: u64,
        duration: i64,
        cliff_duration: Option<i64>,
        is_linear: bool, // whether to release token linearly or not
    },
    ClaimTimeLock {
        time_lock_index: usize, // my token gets unlocked
    },
    CancelTimeLock {
        time_lock_index: usize, // admin can cancel the stream flow before claim date
    },

    SetYieldStrategy {
        token_mint: Pubkey, // token to farm
        strategy_program: Pubkey, // Defi protocol to use
    },

    HarvestYield {
        token_mint: Pubkey, // collect yeiild
    },
    CompoundYield { // automate reinvestment of yield
        token_mint: Pubkey,
    },

    // Dex integration
    JupiterSwap {
        input_mint: Pubkey, // swap token from Sol
        output_mint: Pubkey, // swap token to ETH
        amount: u64, // amount to swap
    },
// TODO: Dex integration
    JupiterRoute { // Execute a complex swap across multiple DEXs
        input_mint: Pubkey,
        output_mint: Pubkey,
        amount: u64,
        route: Vec<u8>, // encoded swap route
    },
    CollectFees,// collect fee which is collected by the vault
    TransferAuthority { // changing vault ownership
        new_authority: Pubkey,
    },
    UpdateEmergencyAdmin {
        new_admin: Pubkey,
    },
    InitializeGovernance { // setup decentralized governance
        voting_token_mint: Pubkey, // token used for voting
        quorm_threshold: u16,
        proposal_threshold: u64,
        voting_period: i64,
        time_lock_delay: i64,
        execution_threshold: u16,
    },
    CreateGovernanceProposal { // create a new proposal
        title: String, // proposal title
        description: String,
        instructions: Vec<GovernanceInstruction>, // action to execute if passed
    },
    CasteVote {
        proposal_id: u64, // which proposal to vote on
        vote_type: VoteType, // For / Against/ Abstain
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
