use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    instruction::{AccountMeta, Instruction},
    transaction::Transaction,
};
use std::str::FromStr;
use borsh::{BorshSerialize, BorshDeserialize};
use hex;

use spl_token;
use std::fs;
use std::env;
use std::io::Write;
use comfy_table::{Table, presets::UTF8_FULL, Cell};
use dirs::home_dir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long)]
    rpc_url: Option<String>,

    #[arg(long)]
    keypair: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage connection profiles (add, list, use, remove)
    Profile {
        #[arg(long)]
        action: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        rpc_url: Option<String>,
        #[arg(long)]
        ws_url: Option<String>,
        #[arg(long)]
        program_id: Option<String>,
        #[arg(long)]
        keypair: Option<String>,
    },
    /// Create a new keypair
    CreateKeypair {
        #[arg(long)]
        output_path: String,
    },
    /// Check account balance
    Balance {
        #[arg(long)]
        account: String,
    },
    /// Get latest blockhash
    Blockhash,
    /// Check transaction status
    TxStatus {
        #[arg(long)]
        signature: String,
    },
    /// Initialize vault
    InitVault {
        #[arg(long)]
        authority: String,
        #[arg(long)]
        emergency_admin: String,
        #[arg(long)]
        bump: u8,
    },
    /// Add supported token
    AddToken {
        #[arg(long)]
        vault: String,
        #[arg(long)]
        mint: String,
        #[arg(long)]
        bump: u8,
    },
    /// Deposit tokens
    Deposit {
        #[arg(long)]
        vault: String,
        #[arg(long)]
        user_token_account: String,
        #[arg(long)]
        vault_token_account: String,
        #[arg(long)]
        amount: u64,
    },
    /// Withdraw tokens
    Withdraw {
        #[arg(long)]
        vault: String,
        #[arg(long)]
        vault_token_account: String,
        #[arg(long)]
        user_token_account: String,
        #[arg(long)]
        amount: u64,
    },
    /// Get vault information
    Info {
        #[arg(long)]
        vault: String,
    },
    /// Initialize multisig
    InitMultisig {
        #[arg(long)]
        vault: String,
        #[arg(long)]
        owners: String,
        #[arg(long)]
        threshold: u64,
        #[arg(long)]
        nonce: u8,
    },
    /// Create multisig transaction
    CreateMultisigTx {
        #[arg(long)]
        vault: String,
        #[arg(long)]
        program_id: String,
        #[arg(long)]
        instruction_data: String,
        #[arg(long)]
        accounts: String,
    },
    /// Approve multisig transaction
    ApproveMultisigTx {
        #[arg(long)]
        vault: String,
        #[arg(long)]
        transaction_id: u64,
    },
    /// Execute multisig transaction
    ExecuteMultisigTx {
        #[arg(long)]
        vault: String,
        #[arg(long)]
        transaction_id: u64,
    },
    /// Set multisig owners
    UpdateMultisigOwners {
        #[arg(long)]
        vault: String,
        #[arg(long)]
        owners: String,
    },
    /// Change multisig threshold
    UpdateMultisigThreshold {
        #[arg(long)]
        vault: String,
        #[arg(long)]
        threshold: u64,
    },
    /// List multisig transactions
    ListMultisigTxs {
        #[arg(long)]
        vault: String,
        #[arg(long, default_value = "10")]
        limit: usize,
    },
    /// Get public key from keypair file
    PubkeyFromKeypair {
        #[arg(long)]
        keypair_path: String,
    },
    /// Create a test transaction with signature
    CreateTestTx {
        #[arg(long)]
        message: Option<String>,
        #[arg(long)]
        keypair: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    // Use RPC_URL from environment variable, fallback to CLI arg, then default
    let rpc_url = std::env::var("RPC_URL")
        .unwrap_or_else(|_| cli.rpc_url.unwrap_or_else(|| "https://api.devnet.solana.com".to_string()));

    // Use PROGRAM_ID from environment variable, fallback to default
    let program_id_str = std::env::var("PROGRAM_ID")
        .unwrap_or_else(|_| "11111111111111111111111111111112".to_string());
    let program_id = Pubkey::from_str(&program_id_str)?;

    let rpc_client = RpcClient::new_with_commitment(
        rpc_url.clone(),
        CommitmentConfig::confirmed(),
    );

    match cli.command {
        Commands::CreateKeypair { output_path } => {
            let keypair = Keypair::new();
            let keypair_data = keypair.to_bytes().to_vec();
            let json_data = serde_json::to_string(&keypair_data)?;
            std::fs::write(&output_path, &json_data)?;
            println!("Created keypair: {}", keypair.pubkey());
            println!("Saved to: {}", output_path);
        }
        Commands::Balance { account } => {
            let pubkey = Pubkey::from_str(&account)?;
            let balance = rpc_client.get_balance(&pubkey)?;
            println!("Balance: {} SOL", balance as f64 / 1_000_000_000.0);
        }
        Commands::Blockhash => {
            let blockhash = rpc_client.get_latest_blockhash()?;
            println!("Recent blockhash: {}", blockhash);
        }
        Commands::TxStatus { signature } => {
            let sig = solana_sdk::signature::Signature::from_str(&signature)?;
            let status = rpc_client.get_signature_status(&sig)?;
            println!("Transaction status: {:?}", status);
        }
        Commands::InitVault { authority, emergency_admin, bump } => {
            let authority_pubkey = Pubkey::from_str(&authority)?;
            let emergency_admin_pubkey = Pubkey::from_str(&emergency_admin)?;

            println!("Initializing vault...");
            println!("Authority: {}", authority_pubkey);
            println!("Emergency Admin: {}", emergency_admin_pubkey);
            println!("Bump: {}", bump);
        }
        Commands::Info { vault } => {
            let vault_pubkey = Pubkey::from_str(&vault)?;
            println!("Vault info for: {}", vault_pubkey);
        }
        Commands::InitMultisig { vault, owners, threshold, nonce } => {
            let vault_pubkey = Pubkey::from_str(&vault)?;
            let owner_list: Vec<Pubkey> = owners.split(',')
                .map(|s| Pubkey::from_str(s.trim()))
                .collect::<Result<Vec<_>, _>>()?;

            println!("Initializing multisig...");
            println!("Vault: {}", vault_pubkey);
            println!("Owners: {:?}", owner_list);
            println!("Threshold: {}", threshold);
            println!("Nonce: {}", nonce);
        }
        Commands::CreateMultisigTx { vault, program_id: target_program, instruction_data, accounts } => {
            let vault_pubkey = Pubkey::from_str(&vault)?;
            let target_program_id = Pubkey::from_str(&target_program)?;

            println!("Creating multisig transaction...");
            println!("Vault: {}", vault_pubkey);
            println!("Target Program: {}", target_program_id);
            println!("Instruction Data: {}", instruction_data);
            println!("Accounts: {}", accounts);
        }
        Commands::ApproveMultisigTx { vault, transaction_id } => {
            let vault_pubkey = Pubkey::from_str(&vault)?;

            println!("Approving multisig transaction...");
            println!("Vault: {}", vault_pubkey);
            println!("Transaction ID: {}", transaction_id);
        }
        Commands::ExecuteMultisigTx { vault, transaction_id } => {
            let vault_pubkey = Pubkey::from_str(&vault)?;

            println!("Executing multisig transaction...");
            println!("Vault: {}", vault_pubkey);
            println!("Transaction ID: {}", transaction_id);
        }
        Commands::ListMultisigTxs { vault, limit } => {
            let vault_pubkey = Pubkey::from_str(&vault)?;

            println!("Listing multisig transactions...");
            println!("Vault: {}", vault_pubkey);
            println!("Limit: {}", limit);
        }
        Commands::PubkeyFromKeypair { keypair_path } => {
            let keypair_data: Vec<u8> = serde_json::from_str(&std::fs::read_to_string(&keypair_path)?)?;
            if keypair_data.len() != 64 {
                return Err("Invalid keypair file: must contain exactly 64 bytes".into());
            }

            // Extract the last 32 bytes (public key) from the 64-byte keypair
            let pubkey_bytes = &keypair_data[32..64];
            let pubkey = Pubkey::new_from_array(pubkey_bytes.try_into().unwrap());

            println!("Public Key: {}", pubkey);
            println!("Keypair file: {}", keypair_path);
        }
        Commands::CreateTestTx { message, keypair } => {
            let memo_program = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")?;
            let default_message = message.unwrap_or_else(|| "Test transaction from vault CLI".to_string());
            let keypair_path = keypair.unwrap_or_else(|| "aditya-keypair.json".to_string());

            // Load keypair from file
            let keypair_data: Vec<u8> = serde_json::from_str(&std::fs::read_to_string(&keypair_path)?)?;
            let signer = Keypair::from_bytes(&keypair_data)?;

            // Create a simple memo instruction
            let memo_ix = Instruction {
                program_id: memo_program,
                accounts: vec![],
                data: default_message.as_bytes().to_vec(),
            };

            // Get recent blockhash
            let recent_blockhash = rpc_client.get_latest_blockhash()?;

            // Create transaction
            let transaction = Transaction::new_signed_with_payer(
                &[memo_ix],
                Some(&signer.pubkey()),
                &[&signer],
                recent_blockhash,
            );

            // Send transaction
            let signature = rpc_client.send_and_confirm_transaction(&transaction)?;

            println!("✅ Transaction created successfully!");
            println!("📝 Message: {}", default_message);
            println!("🔑 Signer: {}", signer.pubkey());
            println!("🔗 Signature: {}", signature);
            println!("🌐 View on Solana Explorer:");
            println!("   https://explorer.solana.com/tx/{}?cluster=devnet", signature);
            println!("   https://solscan.io/tx/{}?cluster=devnet", signature);
        }
        _ => {
            println!("Command not implemented yet");
        }
    }

    Ok(())
}
