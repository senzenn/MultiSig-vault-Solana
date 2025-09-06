use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

// DeFi protocol program IDs (mainnet)
pub mod defi_protocols {
    use super::Pubkey;

    // Orca Whirlpool Program ID
    pub const ORCA_WHIRLPOOL: Pubkey = Pubkey::new_from_array([
        0x77, 0x68, 0x69, 0x72, 0x4c, 0x62, 0x4d, 0x69, 0x69, 0x63, 0x56, 0x64, 0x69, 0x6f, 0x34, 0x71,
        0x76, 0x55, 0x66, 0x4d, 0x35, 0x4b, 0x41, 0x67, 0x36, 0x43, 0x74, 0x38, 0x56, 0x77, 0x70, 0x59,
    ]);

    // Raydium AMM Program ID (V4)
    pub const RAYDIUM_AMM: Pubkey = Pubkey::new_from_array([
        0x36, 0x37, 0x35, 0x6b, 0x50, 0x58, 0x39, 0x4d, 0x48, 0x54, 0x6a, 0x53, 0x32, 0x7a, 0x74, 0x31,
        0x71, 0x66, 0x72, 0x31, 0x4e, 0x59, 0x48, 0x75, 0x7a, 0x65, 0x4c, 0x58, 0x66, 0x51, 0x4d, 0x39,
    ]);

    // Saber Protocol Program ID
    pub const SABER_PROTOCOL: Pubkey = Pubkey::new_from_array([
        0x53, 0x53, 0x77, 0x70, 0x6b, 0x45, 0x45, 0x63, 0x62, 0x55, 0x71, 0x78, 0x34, 0x76, 0x74, 0x6f,
        0x45, 0x42, 0x79, 0x46, 0x6a, 0x53, 0x6b, 0x68, 0x4b, 0x64, 0x43, 0x54, 0x38, 0x36, 0x32, 0x44,
    ]);

    // Jupiter Aggregator Program ID
    pub const JUPITER_AGGREGATOR: Pubkey = Pubkey::new_from_array([
        0x4a, 0x55, 0x50, 0x36, 0x4c, 0x6b, 0x62, 0x5a, 0x62, 0x6a, 0x53, 0x31, 0x6a, 0x4b, 0x4b, 0x77,
        0x61, 0x70, 0x64, 0x48, 0x4e, 0x79, 0x37, 0x34, 0x7a, 0x63, 0x5a, 0x33, 0x74, 0x4c, 0x55, 0x5a,
    ]);
}

// Generic yield farming protocol interface
pub trait YieldProtocol {
    fn deposit_instruction(
        &self,
        vault_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, ProgramError>;

    fn withdraw_instruction(
        &self,
        vault_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, ProgramError>;

    fn harvest_instruction(
        &self,
        vault_token_account: &Pubkey,
        reward_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
    ) -> Result<Instruction, ProgramError>;

    fn get_protocol_id(&self) -> Pubkey;
}

// Orca Whirlpool integration
pub struct OrcaProtocol;

impl YieldProtocol for OrcaProtocol {
    fn deposit_instruction(
        &self,
        vault_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(Pubkey::new_from_array([0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x6b, 0x65, 0x67, 0x51, 0x66, 0x65, 0x5a, 0x79, 0x69, 0x4e, 0x77, 0x41, 0x4a, 0x62, 0x4e, 0x62, 0x47, 0x4b, 0x50, 0x46, 0x58, 0x43, 0x57, 0x75, 0x42, 0x76, 0x66]), false),
            AccountMeta::new_readonly(defi_protocols::ORCA_WHIRLPOOL, false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::ORCA_WHIRLPOOL,
            accounts,
            data: vec![1, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8],
        })
    }

    fn withdraw_instruction(
        &self,
        vault_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(Pubkey::new_from_array([0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x6b, 0x65, 0x67, 0x51, 0x66, 0x65, 0x5a, 0x79, 0x69, 0x4e, 0x77, 0x41, 0x4a, 0x62, 0x4e, 0x62, 0x47, 0x4b, 0x50, 0x46, 0x58, 0x43, 0x57, 0x75, 0x42, 0x76, 0x66]), false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::ORCA_WHIRLPOOL,
            accounts,
            data: vec![2, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8],
        })
    }

    fn harvest_instruction(
        &self,
        vault_token_account: &Pubkey,
        reward_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*reward_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(Pubkey::new_from_array([0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x6b, 0x65, 0x67, 0x51, 0x66, 0x65, 0x5a, 0x79, 0x69, 0x4e, 0x77, 0x41, 0x4a, 0x62, 0x4e, 0x62, 0x47, 0x4b, 0x50, 0x46, 0x58, 0x43, 0x57, 0x75, 0x42, 0x76, 0x66]), false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::ORCA_WHIRLPOOL,
            accounts,
            data: vec![3],
        })
    }

    fn get_protocol_id(&self) -> Pubkey {
        defi_protocols::ORCA_WHIRLPOOL
    }
}

// Raydium AMM integration
pub struct RaydiumProtocol;

impl YieldProtocol for RaydiumProtocol {
    fn deposit_instruction(
        &self,
        vault_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(Pubkey::new_from_array([0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x6b, 0x65, 0x67, 0x51, 0x66, 0x65, 0x5a, 0x79, 0x69, 0x4e, 0x77, 0x41, 0x4a, 0x62, 0x4e, 0x62, 0x47, 0x4b, 0x50, 0x46, 0x58, 0x43, 0x57, 0x75, 0x42, 0x76, 0x66]), false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::RAYDIUM_AMM,
            accounts,
            data: vec![10, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8],
        })
    }

    fn withdraw_instruction(
        &self,
        vault_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(Pubkey::new_from_array([0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x6b, 0x65, 0x67, 0x51, 0x66, 0x65, 0x5a, 0x79, 0x69, 0x4e, 0x77, 0x41, 0x4a, 0x62, 0x4e, 0x62, 0x47, 0x4b, 0x50, 0x46, 0x58, 0x43, 0x57, 0x75, 0x42, 0x76, 0x66]), false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::RAYDIUM_AMM,
            accounts,
            data: vec![11, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8],
        })
    }

    fn harvest_instruction(
        &self,
        vault_token_account: &Pubkey,
        reward_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*reward_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(Pubkey::new_from_array([0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x6b, 0x65, 0x67, 0x51, 0x66, 0x65, 0x5a, 0x79, 0x69, 0x4e, 0x77, 0x41, 0x4a, 0x62, 0x4e, 0x62, 0x47, 0x4b, 0x50, 0x46, 0x58, 0x43, 0x57, 0x75, 0x42, 0x76, 0x66]), false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::RAYDIUM_AMM,
            accounts,
            data: vec![12],
        })
    }

    fn get_protocol_id(&self) -> Pubkey {
        defi_protocols::RAYDIUM_AMM
    }
}

// Saber Protocol integration
pub struct SaberProtocol;

impl YieldProtocol for SaberProtocol {
    fn deposit_instruction(
        &self,
        vault_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(Pubkey::new_from_array([0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x6b, 0x65, 0x67, 0x51, 0x66, 0x65, 0x5a, 0x79, 0x69, 0x4e, 0x77, 0x41, 0x4a, 0x62, 0x4e, 0x62, 0x47, 0x4b, 0x50, 0x46, 0x58, 0x43, 0x57, 0x75, 0x42, 0x76, 0x66]), false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::SABER_PROTOCOL,
            accounts,
            data: vec![20, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8],
        })
    }

    fn withdraw_instruction(
        &self,
        vault_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(Pubkey::new_from_array([0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x6b, 0x65, 0x67, 0x51, 0x66, 0x65, 0x5a, 0x79, 0x69, 0x4e, 0x77, 0x41, 0x4a, 0x62, 0x4e, 0x62, 0x47, 0x4b, 0x50, 0x46, 0x58, 0x43, 0x57, 0x75, 0x42, 0x76, 0x66]), false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::SABER_PROTOCOL,
            accounts,
            data: vec![21, amount as u8, (amount >> 8) as u8, (amount >> 16) as u8, (amount >> 24) as u8],
        })
    }

    fn harvest_instruction(
        &self,
        vault_token_account: &Pubkey,
        reward_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*reward_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(Pubkey::new_from_array([0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x6b, 0x65, 0x67, 0x51, 0x66, 0x65, 0x5a, 0x79, 0x69, 0x4e, 0x77, 0x41, 0x4a, 0x62, 0x4e, 0x62, 0x47, 0x4b, 0x50, 0x46, 0x58, 0x43, 0x57, 0x75, 0x42, 0x76, 0x66]), false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::SABER_PROTOCOL,
            accounts,
            data: vec![22],
        })
    }

    fn get_protocol_id(&self) -> Pubkey {
        defi_protocols::SABER_PROTOCOL
    }
}

// Jupiter Aggregator integration
pub struct JupiterProtocol;

impl YieldProtocol for JupiterProtocol {
    fn deposit_instruction(
        &self,
        vault_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(defi_protocols::JUPITER_AGGREGATOR, false),
            AccountMeta::new_readonly(Pubkey::new_from_array([0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x6b, 0x65, 0x67, 0x51, 0x66, 0x65, 0x5a, 0x79, 0x69, 0x4e, 0x77, 0x41, 0x4a, 0x62, 0x4e, 0x62, 0x47, 0x4b, 0x50, 0x46, 0x58, 0x43, 0x57, 0x75, 0x42, 0x76, 0x66]), false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::JUPITER_AGGREGATOR,
            accounts,
            data: vec![
                0,
                (amount & 0xFF) as u8,
                ((amount >> 8) & 0xFF) as u8,
                ((amount >> 16) & 0xFF) as u8,
                ((amount >> 24) & 0xFF) as u8,
                ((amount >> 32) & 0xFF) as u8,
                ((amount >> 40) & 0xFF) as u8,
                ((amount >> 48) & 0xFF) as u8,
                ((amount >> 56) & 0xFF) as u8,
            ],
        })
    }

    fn withdraw_instruction(
        &self,
        vault_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(defi_protocols::JUPITER_AGGREGATOR, false),
            AccountMeta::new_readonly(Pubkey::new_from_array([0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x6b, 0x65, 0x67, 0x51, 0x66, 0x65, 0x5a, 0x79, 0x69, 0x4e, 0x77, 0x41, 0x4a, 0x62, 0x4e, 0x62, 0x47, 0x4b, 0x50, 0x46, 0x58, 0x43, 0x57, 0x75, 0x42, 0x76, 0x66]), false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::JUPITER_AGGREGATOR,
            accounts,
            data: vec![
                1,
                (amount & 0xFF) as u8,
                ((amount >> 8) & 0xFF) as u8,
                ((amount >> 16) & 0xFF) as u8,
                ((amount >> 24) & 0xFF) as u8,
                ((amount >> 32) & 0xFF) as u8,
                ((amount >> 40) & 0xFF) as u8,
                ((amount >> 48) & 0xFF) as u8,
                ((amount >> 56) & 0xFF) as u8,
            ],
        })
    }

    fn harvest_instruction(
        &self,
        vault_token_account: &Pubkey,
        reward_token_account: &Pubkey,
        strategy_account: &Pubkey,
        authority: &Pubkey,
    ) -> Result<Instruction, ProgramError> {
        let accounts = vec![
            AccountMeta::new(*vault_token_account, false),
            AccountMeta::new(*reward_token_account, false),
            AccountMeta::new(*strategy_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(defi_protocols::JUPITER_AGGREGATOR, false),
        ];

        Ok(Instruction {
            program_id: defi_protocols::JUPITER_AGGREGATOR,
            accounts,
            data: vec![2],
        })
    }

    fn get_protocol_id(&self) -> Pubkey {
        defi_protocols::JUPITER_AGGREGATOR
    }
}

// Protocol registry lookup
pub fn get_protocol(protocol_id: &Pubkey) -> Option<Box<dyn YieldProtocol>> {
    match *protocol_id {
        defi_protocols::ORCA_WHIRLPOOL => Some(Box::new(OrcaProtocol)),
        defi_protocols::RAYDIUM_AMM => Some(Box::new(RaydiumProtocol)),
        defi_protocols::SABER_PROTOCOL => Some(Box::new(SaberProtocol)),
        defi_protocols::JUPITER_AGGREGATOR => Some(Box::new(JupiterProtocol)),
        _ => None,
    }
}