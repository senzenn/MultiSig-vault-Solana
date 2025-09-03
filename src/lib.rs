use solana_program::{
  entrypoint,
  pubkey::Pubkey
};

solana_program::declare_program!("VAULT1111111111111111111111111111111111111111111111111111111111111111");

pub mod instruction;
pub mod defi;
pub mod processor;
pub mod state;
pub mod events;


// reexport
pub use instruction::*;
pub use defi::*;
pub use processor::*;
pub use state::*;
pub use events::*;

//  emit_event:-> think of it like pusher and logs it with msg macro simple :)
#[macro_export]  // this is a way of saying like export emit_event  which means you can use this macro across the project files.
macro_rules! emit_event {
  ($event:ident, $data:expr) => {
    msg!("EVENT: {}",serde_json::to_string(&$event).unwrap_or_else(|_| "Failed to serialize event".to_string()));
  };
}

// helper structure for all vault events

pub fn create_base_event(vault: Pubkey,authority: Pubkey, event_type: &str, clock: &solana_program::clock::Clock) -> VaultEvent{
  VaultEvent {
    vault: vault, // pubKey of vault acc
    authority: authority, // pubkey of the authority
    event_type: event_type.to_string(), // think of it like dep| with | transfer
    clock: clock.unix_timestamp, // timestamp but in solana clock
  }
}

entrypoint!(process_instruction);
// entrypoint is a solana program macro  which tells the  sol runtime to which function to call when program is invoked

