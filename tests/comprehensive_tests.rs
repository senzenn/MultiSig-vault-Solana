#[cfg(test)]
mod comprehensive_tests {
    use solana_program::{
        pubkey::Pubkey,
        instruction::{AccountMeta, Instruction},
        system_program,
        system_instruction,
        clock::Clock,
        sysvar::Sysvar,
        rent::Rent,
        program_pack::Pack,
    };
    use solana_program_test::*;
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
    };
    use spl_token::{instruction as token_instruction, state::Account as TokenAccount};
    use spl_associated_token_account::instruction as ata_instruction;
    use vault_program::{
        instruction::VaultInstruction,
        state::{Vault, MultiSig, MultiSigTransaction, TransactionAccount, FeeConfig, SupportedToken, TimeLock, GovernanceConfig, GovernanceProposal, VoteType},
        processor::process_instruction,
        PROGRAM_ID,
    };
    use borsh::{BorshSerialize, BorshDeserialize};
    use std::mem;

    fn program_test() -> ProgramTest {
        ProgramTest::new(
            "vault_program",
            PROGRAM_ID,
            Some(vault_program::process_instruction),
        )
    }

    // ===== VAULT CREATION TESTS =====

    async fn create_vault(banks_client: &mut BanksClient, payer: &Keypair, recent_blockhash: &solana_sdk::hash::Hash) -> Pubkey {
        let vault_keypair = Keypair::new();
        let vault_pubkey = vault_keypair.pubkey();
        let mint_pubkey = Pubkey::new_unique();
        let vault_token_account = spl_associated_token_account::get_associated_token_address(&vault_pubkey, &mint_pubkey);

        // Calculate vault size
        let vault_size = mem::size_of::<Vault>() as u64;
        let rent = banks_client.get_rent().await.unwrap();
        let vault_rent = rent.minimum_balance(vault_size as usize);

        // Create vault account
        let create_vault_ix = system_instruction::create_account(
            &payer.pubkey(),
            &vault_pubkey,
            vault_rent,
            vault_size,
            &PROGRAM_ID,
        );

        // Initialize vault
        let initialize_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(mint_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new(vault_token_account, false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            ],
            data: VaultInstruction::Initialize { bump: 0 }
                .try_to_vec()
                .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[create_vault_ix, initialize_ix],
            Some(&payer.pubkey()),
            &[&payer, &vault_keypair],
            *recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();
        vault_pubkey
    }

    #[tokio::test]
    async fn test_01_create_vault() {
        println!("🔧 Testing: Create Vault");
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;

        // Verify vault was created
        let vault_account = banks_client
            .get_account(vault_pubkey)
            .await
            .unwrap()
            .unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert_eq!(vault.authority, payer.pubkey());
        assert_eq!(vault.emergency_admin, payer.pubkey());
        assert!(!vault.paused);

        println!("✅ Vault creation successful");
    }

    // ===== MULTISIGN TESTS =====

    #[tokio::test]
    async fn test_02_create_multisig() {
        println!("🔐 Testing: Create Multisig");
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;

        // Create multisig owners
        let owner1 = Keypair::new();
        let owner2 = Keypair::new();
        let owner3 = Keypair::new();
        let owners = vec![payer.pubkey(), owner1.pubkey(), owner2.pubkey(), owner3.pubkey()];
        let threshold = 3u64;

        let initialize_multisig_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::InitializeMultiSig {
                owners: owners.clone(),
                threshold,
                nonce: 0,
            }
            .try_to_vec()
            .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[initialize_multisig_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify multisig was created
        let vault_account = banks_client
            .get_account(vault_pubkey)
            .await
            .unwrap()
            .unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert!(vault.multi_sig.is_some());
        let multisig = vault.multi_sig.as_ref().unwrap();
        assert_eq!(multisig.owners, owners);
        assert_eq!(multisig.threshold, threshold);

        println!("✅ Multisig creation successful");
    }

    // ===== TIMELOCK TESTS =====

    #[tokio::test]
    async fn test_03_create_timelock() {
        println!("⏰ Testing: Create Timelock");
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;
        let beneficiary = Keypair::new();
        let amount = 1_000_000u64; // 0.001 SOL
        let duration = 86400i64; // 24 hours in seconds
        let cliff_duration = Some(43200i64); // 12 hours cliff

        let create_timelock_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::CreateTimeLock {
                beneficiary: beneficiary.pubkey(),
                amount,
                duration,
                cliff_duration,
                is_linear: true,
            }
            .try_to_vec()
            .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[create_timelock_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify timelock was created
        let vault_account = banks_client
            .get_account(vault_pubkey)
            .await
            .unwrap()
            .unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert_eq!(vault.time_locks.len(), 1);
        let timelock = &vault.time_locks[0];
        assert_eq!(timelock.beneficiary, beneficiary.pubkey());
        assert_eq!(timelock.amount, amount);

        println!("✅ Timelock creation successful");
    }

    // ===== GOVERNANCE TESTS =====

    #[tokio::test]
    async fn test_04_create_governance() {
        println!("🏛️ Testing: Create Governance");
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;
        let voting_token_mint = Pubkey::new_unique();
        let quorum_threshold = 60u16; // 60%
        let proposal_threshold = 1000u64; // 1000 tokens
        let voting_period = 604800i64; // 7 days
        let time_lock_delay = 172800i64; // 2 days
        let execution_threshold = 50u16; // 50%

        let initialize_governance_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::InitializeGovernance {
                voting_token_mint,
                quorm_threshold: quorum_threshold,
                proposal_threshold,
                voting_period,
                time_lock_delay,
                execution_threshold,
            }
            .try_to_vec()
            .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[initialize_governance_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify governance was created
        let vault_account = banks_client
            .get_account(vault_pubkey)
            .await
            .unwrap()
            .unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert!(vault.governance_config.is_some());
        let governance = vault.governance_config.as_ref().unwrap();
        assert_eq!(governance.voting_token_mint, voting_token_mint);
        assert_eq!(governance.quorum_threshold, quorum_threshold);

        println!("✅ Governance creation successful");
    }

    // ===== YIELD FARMING TESTS =====

    #[tokio::test]
    async fn test_05_create_yield_farming() {
        println!("🌾 Testing: Create Yield Farming");
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;
        let token_mint = Pubkey::new_unique();
        let strategy_program = Pubkey::new_unique();

        let set_yield_strategy_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::SetYieldStrategy {
                token_mint,
                strategy_program,
            }
            .try_to_vec()
            .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[set_yield_strategy_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify yield strategy was set
        let vault_account = banks_client
            .get_account(vault_pubkey)
            .await
            .unwrap()
            .unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert_eq!(vault.yield_strategies.len(), 1);
        let strategy = &vault.yield_strategies[0];
        assert_eq!(strategy.token_mint, token_mint);
        assert_eq!(strategy.strategy_program, strategy_program);

        println!("✅ Yield farming setup successful");
    }

    // ===== MULTITOKEN TESTS =====

    #[tokio::test]
    async fn test_06_create_multitoken() {
        println!("🪙 Testing: Create Multitoken Support");
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;
        let token_mint = Pubkey::new_unique();

        // First add supported token
        let add_token_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::AddSupportedToken {
                mint: token_mint,
                bump: 0,
            }
            .try_to_vec()
            .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[add_token_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify token was added
        let vault_account = banks_client
            .get_account(vault_pubkey)
            .await
            .unwrap()
            .unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert_eq!(vault.supported_tokens.len(), 1);
        let supported_token = &vault.supported_tokens[0];
        assert_eq!(supported_token.mint, token_mint);
        assert!(supported_token.is_active);

        println!("✅ Multitoken support successful");
    }

    // ===== EMERGENCY TESTS =====

    #[tokio::test]
    async fn test_07_create_emergency() {
        println!("🚨 Testing: Emergency Features");
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;

        // Test pause vault
        let pause_vault_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
            ],
            data: VaultInstruction::PauseVault
                .try_to_vec()
                .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[pause_vault_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify vault was paused
        let vault_account = banks_client
            .get_account(vault_pubkey)
            .await
            .unwrap()
            .unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert!(vault.paused);

        // Test unpause vault
        let unpause_vault_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
            ],
            data: VaultInstruction::UnpauseVault
                .try_to_vec()
                .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[unpause_vault_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify vault was unpaused
        let vault_account = banks_client
            .get_account(vault_pubkey)
            .await
            .unwrap()
            .unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert!(!vault.paused);

        println!("✅ Emergency features successful");
    }

    // ===== ADMIN TESTS =====

    #[tokio::test]
    async fn test_08_admin_features() {
        println!("👑 Testing: Admin Features");
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;
        let new_authority = Keypair::new();
        let new_admin = Keypair::new();

        // Test transfer authority
        let transfer_authority_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
            ],
            data: VaultInstruction::TransferAuthority {
                new_authority: new_authority.pubkey(),
            }
            .try_to_vec()
            .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[transfer_authority_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify authority was transferred
        let vault_account = banks_client
            .get_account(vault_pubkey)
            .await
            .unwrap()
            .unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert_eq!(vault.authority, new_authority.pubkey());

        // Test update emergency admin
        let update_admin_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(new_authority.pubkey(), true),
            ],
            data: VaultInstruction::UpdateEmergencyAdmin {
                new_admin: new_admin.pubkey(),
            }
            .try_to_vec()
            .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[update_admin_ix],
            Some(&payer.pubkey()),
            &[&payer, &new_authority],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify admin was updated
        let vault_account = banks_client
            .get_account(vault_pubkey)
            .await
            .unwrap()
            .unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert_eq!(vault.emergency_admin, new_admin.pubkey());

        println!("✅ Admin features successful");
    }

    // ===== COMPREHENSIVE INTEGRATION TEST =====

    #[tokio::test]
    async fn test_09_comprehensive_vault_workflow() {
        println!("🚀 Testing: Comprehensive Vault Workflow");
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;

        // Test deposit
        let deposit_amount = 500_000u64; // 0.0005 SOL
        let deposit_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::Deposite { amount: deposit_amount }
                .try_to_vec()
                .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[deposit_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Test withdraw
        let withdraw_amount = 200_000u64; // 0.0002 SOL
        let withdraw_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::Withdraw { amount: withdraw_amount }
                .try_to_vec()
                .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[withdraw_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Test transfer
        let recipient = Keypair::new();
        let transfer_amount = 100_000u64; // 0.0001 SOL
        let transfer_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::Transfer {
                recipient: recipient.pubkey(),
                amount: transfer_amount,
            }
            .try_to_vec()
            .unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[transfer_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        println!("✅ Comprehensive vault workflow successful");
    }
}
