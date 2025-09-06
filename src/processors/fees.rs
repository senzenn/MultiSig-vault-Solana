use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::state::Vault;

pub fn process_update_fee_config(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    deposit_fee_bps: u16,
    withdrawal_fee_bps: u16,
    fee_recipient: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_data = vault_account.data.borrow();
    let mut vault = Vault::try_from_slice(&vault_data)?;

    let is_authorized = if let Some(multi_sig) = &vault.multi_sig {
        multi_sig.authorities.contains(authority.key)
    } else {
        vault.authority == *authority.key
    };

    if !is_authorized {
        return Err(ProgramError::InvalidAccountData);
    }

    vault.fee_config = crate::state::FeeConfig {
        deposit_fee_bps,
        withdrawal_fee_bps,
        fee_recipient,
    };

    drop(vault_data);
    vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    msg!("Fee config updated: deposit {} bps, withdrawal {} bps", deposit_fee_bps, withdrawal_fee_bps);
    Ok(())
}

pub fn process_collect_fees(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    msg!("Fee collection - not yet implemented");
    Ok(())
}
