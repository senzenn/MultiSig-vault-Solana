use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{instruction::{AccountMeta, Instruction}, program_error::ProgramError, pubkey::Pubkey};

pub mod ids {
    use solana_program::pubkey::Pubkey;
    pub const ORCA_WHIRLPOOL: Pubkey = Pubkey::new_from_array([
        0x9b, 0xb4, 0x5b, 0x8c, 0x3a, 0x8e, 0x8e, 0x4a, 0x1b, 0x6f, 0x8e, 0xa9, 0x7a, 0x2b, 0x3d, 0x5f,
        0x8c, 0x9e, 0x4b, 0x7d, 0x2b, 0x8c, 0x6e, 0x9e, 0x1b, 0x5f, 0x9c, 0x2d, 0x7a, 0x8e, 0x4b, 0x6f,
    ]);
    pub const RAYDIUM_AMM: Pubkey = Pubkey::new_from_array([
        0x9c, 0xb4, 0x5b, 0x8c, 0x3a, 0x8e, 0x8e, 0x4a, 0x1b, 0x6f, 0x8e, 0xa9, 0x7a, 0x2b, 0x3d, 0x5f,
        0x8c, 0x9e, 0x4b, 0x7d, 0x2b, 0x8c, 0x6e, 0x9e, 0x1b, 0x5f, 0x9c, 0x2d, 0x7a, 0x8e, 0x4b, 0x70,
    ]);
    pub const SABER_PROTOCOL: Pubkey = Pubkey::new_from_array([
        0x9d, 0xb4, 0x5b, 0x8c, 0x3a, 0x8e, 0x8e, 0x4a, 0x1b, 0x6f, 0x8e, 0xa9, 0x7a, 0x2b, 0x3d, 0x5f,
        0x8c, 0x9e, 0x4b, 0x7d, 0x2b, 0x8c, 0x6e, 0x9e, 0x1b, 0x5f, 0x9c, 0x2d, 0x7a, 0x8e, 0x4b, 0x71,
    ]);
    pub const JUPITER_AGGREGATOR: Pubkey = Pubkey::new_from_array([
        0x9e, 0xb4, 0x5b, 0x8c, 0x3a, 0x8e, 0x8e, 0x4a, 0x1b, 0x6f, 0x8e, 0xa9, 0x7a, 0x2b, 0x3d, 0x5f,
        0x8c, 0x9e, 0x4b, 0x7d, 0x2b, 0x8c, 0x6e, 0x9e, 0x1b, 0x5f, 0x9c, 0x2d, 0x7a, 0x8e, 0x4b, 0x72,
    ]);
}

pub trait YieldProtocol {
    fn deposit_instruction(&self, vault_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey, amount: u64) -> Result<Instruction, ProgramError>;
    fn withdraw_instruction(&self, vault_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey, amount: u64) -> Result<Instruction, ProgramError>;
    fn harvest_instruction(&self, vault_token_account: &Pubkey, reward_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey) -> Result<Instruction, ProgramError>;
    fn get_protocol_id(&self) -> Pubkey;
}

pub struct OrcaProtocol;
impl YieldProtocol for OrcaProtocol {
    fn deposit_instruction(&self, vault_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey, amount: u64) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(ids::ORCA_WHIRLPOOL, false),
        ];
        Ok(Instruction { program_id: ids::ORCA_WHIRLPOOL, accounts, data: vec![1, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8] })
    }
    fn withdraw_instruction(&self, vault_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey, amount: u64) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ];
        Ok(Instruction { program_id: ids::RAYDIUM_AMM, accounts, data: vec![2, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8] })
    }
    fn harvest_instruction(&self, vault_token_account: &Pubkey, reward_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*reward_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ];
        Ok(Instruction { program_id: ids::ORCA_WHIRLPOOL, accounts, data: vec![3] })
    }
    fn get_protocol_id(&self) -> Pubkey { ids::ORCA_WHIRLPOOL }
}

pub struct RaydiumProtocol;
impl YieldProtocol for RaydiumProtocol {
    fn deposit_instruction(&self, vault_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey, amount: u64) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ];
        Ok(Instruction { program_id: ids::RAYDIUM_AMM, accounts, data: vec![10, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8] })
    }
    fn withdraw_instruction(&self, vault_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey, amount: u64) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ];
        Ok(Instruction { program_id: ids::RAYDIUM_AMM, accounts, data: vec![11, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8] })
    }
    fn harvest_instruction(&self, vault_token_account: &Pubkey, reward_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*reward_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ];
        Ok(Instruction { program_id: ids::RAYDIUM_AMM, accounts, data: vec![12] })
    }
    fn get_protocol_id(&self) -> Pubkey { ids::RAYDIUM_AMM }
}

pub struct SaberProtocol;
impl YieldProtocol for SaberProtocol {
    fn deposit_instruction(&self, vault_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey, amount: u64) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ];
        Ok(Instruction { program_id: ids::SABER_PROTOCOL, accounts, data: vec![20, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8] })
    }
    fn withdraw_instruction(&self, vault_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey, amount: u64) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ];
        Ok(Instruction { program_id: ids::SABER_PROTOCOL, accounts, data: vec![21, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8] })
    }
    fn harvest_instruction(&self, vault_token_account: &Pubkey, reward_token_account: &Pubkey, strategy_account: &Pubkey, authority: &Pubkey) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*reward_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ];
        Ok(Instruction { program_id: ids::SABER_PROTOCOL, accounts, data: vec![22] })
    }
    fn get_protocol_id(&self) -> Pubkey { ids::SABER_PROTOCOL }
}

pub struct JupiterProtocol;
impl JupiterProtocol {
    pub fn swap_instruction(vault_account: &Pubkey, user_account: &Pubkey, token_mint: &Pubkey, amount: u64) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_account, false),
            AccountMeta::new(*user_account, true),
            AccountMeta::new_readonly(*token_mint, false),
            AccountMeta::new_readonly(ids::JUPITER_AGGREGATOR, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ];
        Ok(Instruction { program_id: ids::JUPITER_AGGREGATOR, accounts, data: vec![0,
            (amount & 0xFF) as u8, ((amount >> 8) & 0xFF) as u8, ((amount >> 16) & 0xFF) as u8, ((amount >> 24) & 0xFF) as u8,
            ((amount >> 32) & 0xFF) as u8, ((amount >> 40) & 0xFF) as u8, ((amount >> 48) & 0xFF) as u8, ((amount >> 56) & 0xFF) as u8] })
    }
}

pub fn get_protocol(protocol_id: &Pubkey) -> Option<Box<dyn YieldProtocol>> {
    match *protocol_id {
        ids::ORCA_WHIRLPOOL => Some(Box::new(OrcaProtocol)),
        ids::RAYDIUM_AMM => Some(Box::new(RaydiumProtocol)),
        ids::SABER_PROTOCOL => Some(Box::new(SaberProtocol)),
        _ => None,
    }
}


