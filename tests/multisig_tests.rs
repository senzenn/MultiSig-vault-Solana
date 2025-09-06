#[cfg(test)]
mod tests {
    use solana_program::{
        pubkey::Pubkey,
        instruction::{AccountMeta, Instruction},
        system_program,
    };
    use solana_program_test::*;
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
    };
    use vault_program::{
        instruction::VaultInstruction,
        state::{Vault, TransactionAccount},
        processor::process_instruction,
        PROGRAM_ID,
    };
    use borsh::{BorshSerialize, BorshDeserialize};

    fn program_test() -> ProgramTest {
        ProgramTest::new(
            "vault_program",
            PROGRAM_ID,
            None, // We'll use the default processor
        )
    }

    async fn create_vault(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        recent_blockhash: &solana_sdk::hash::Hash,
    ) -> Pubkey {
        let vault_keypair = Keypair::new();
        let vault_pubkey = vault_keypair.pubkey();

        // Create vault account
        let vault_size = std::mem::size_of::<Vault>() + 1024; // Extra space for multisig data
        let rent = banks_client.get_rent().await.unwrap();
        let vault_rent = rent.minimum_balance(vault_size);

        let create_vault_ix = solana_program::system_instruction::create_account(
            &payer.pubkey(),
            &vault_pubkey,
            vault_rent,
            vault_size as u64,
            &PROGRAM_ID,
        );

        let initialize_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true), // authority
                AccountMeta::new_readonly(payer.pubkey(), false), // emergency_admin
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
                AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::Initialize { bump: 0 }.try_to_vec().unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[create_vault_ix, initialize_ix],
            Some(&payer.pubkey()),
            &[payer, &vault_keypair],
            *recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();
        vault_pubkey
    }

    #[tokio::test]
    async fn test_multisig_initialization() {
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;

        // Create multisig owners
        let owner1 = Keypair::new();
        let owner2 = Keypair::new();
        let owner3 = Keypair::new();
        let owners = vec![owner1.pubkey(), owner2.pubkey(), owner3.pubkey()];
        let threshold = 2u64;
        let nonce = 0u8;

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
                nonce,
            }.try_to_vec().unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[initialize_multisig_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify multisig was created
        let vault_account = banks_client.get_account(vault_pubkey).await.unwrap().unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert!(vault.multi_sig.is_some());
        let multisig = vault.multi_sig.as_ref().unwrap();
        assert_eq!(multisig.owners, owners);
        assert_eq!(multisig.threshold, threshold);
        assert_eq!(multisig.nonce, nonce);
    }

    #[tokio::test]
    async fn test_multisig_initialization_validation() {
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;

        // Test invalid threshold (0)
        let owners = vec![payer.pubkey()];
        let initialize_multisig_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::InitializeMultiSig {
                owners: owners.clone(),
                threshold: 0, // Invalid threshold
                nonce: 0,
            }.try_to_vec().unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[initialize_multisig_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        // Should fail with InvalidThreshold error
        assert!(banks_client.process_transaction(transaction).await.is_err());
    }

    #[tokio::test]
    async fn test_create_multisig_transaction() {
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;

        // Initialize multisig first
        let owner1 = Keypair::new();
        let owner2 = Keypair::new();
        let owners = vec![payer.pubkey(), owner1.pubkey(), owner2.pubkey()];
        let threshold = 2u64;

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
            }.try_to_vec().unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[initialize_multisig_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Create a transaction to transfer SOL
        let recipient = Keypair::new();
        let transfer_amount = 1_000_000; // 0.001 SOL

        let transfer_ix = solana_program::system_instruction::transfer(
            &vault_pubkey, // This will be replaced by multisig signer
            &recipient.pubkey(),
            transfer_amount,
        );

        let transaction_accounts = vec![
            TransactionAccount {
                pubkey: vault_pubkey,
                is_signer: true, // This will be the multisig signer
                is_writable: true,
            },
            TransactionAccount {
                pubkey: recipient.pubkey(),
                is_signer: false,
                is_writable: true,
            },
        ];

        let create_transaction_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::CreateMultiSigTransaction {
                program_id: system_program::id(),
                accounts: transaction_accounts,
                data: transfer_ix.data,
            }.try_to_vec().unwrap(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[create_transaction_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify transaction was created
        let vault_account = banks_client.get_account(vault_pubkey).await.unwrap().unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        assert_eq!(vault.multi_sig_transactions.len(), 1);
        let tx = &vault.multi_sig_transactions[0];
        assert_eq!(tx.program_id, system_program::id());
        assert_eq!(tx.proposer, payer.pubkey());
        assert!(!tx.did_execute);
        assert_eq!(tx.signers.len(), 3); // 3 owners
        assert_eq!(tx.signers[0], true); // First owner (payer) auto-approved
        assert_eq!(tx.signers[1], false); // Other owners not approved yet
        assert_eq!(tx.signers[2], false);
    }

    #[tokio::test]
    async fn test_approve_multisig_transaction() {
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;

        // Initialize multisig
        let owner1 = Keypair::new();
        let owner2 = Keypair::new();
        let owners = vec![payer.pubkey(), owner1.pubkey(), owner2.pubkey()];
        let threshold = 2u64;

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
            }.try_to_vec().unwrap(),
        };

        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[initialize_multisig_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        )).await.unwrap();

        // Create transaction
        let recipient = Keypair::new();
        let transfer_ix = solana_program::system_instruction::transfer(
            &vault_pubkey,
            &recipient.pubkey(),
            1_000_000,
        );

        let transaction_accounts = vec![
            TransactionAccount {
                pubkey: vault_pubkey,
                is_signer: true,
                is_writable: true,
            },
            TransactionAccount {
                pubkey: recipient.pubkey(),
                is_signer: false,
                is_writable: true,
            },
        ];

        let create_transaction_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::CreateMultiSigTransaction {
                program_id: system_program::id(),
                accounts: transaction_accounts,
                data: transfer_ix.data,
            }.try_to_vec().unwrap(),
        };

        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[create_transaction_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        )).await.unwrap();

        // Approve transaction with owner1
        let approve_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(owner1.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::ApproveMultiSigTransaction {
                transaction_id: 0,
            }.try_to_vec().unwrap(),
        };

        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[approve_ix],
            Some(&payer.pubkey()),
            &[&payer, &owner1],
            recent_blockhash,
        )).await.unwrap();

        // Verify approval
        let vault_account = banks_client.get_account(vault_pubkey).await.unwrap().unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        let tx = &vault.multi_sig_transactions[0];
        assert_eq!(tx.signers[0], true); // payer (auto-approved)
        assert_eq!(tx.signers[1], true); // owner1 (just approved)
        assert_eq!(tx.signers[2], false); // owner2 (not approved)
    }

    #[tokio::test]
    async fn test_execute_multisig_transaction() {
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;

        // Fund the vault
        let fund_amount = 10_000_000; // 0.01 SOL
        let fund_ix = solana_program::system_instruction::transfer(
            &payer.pubkey(),
            &vault_pubkey,
            fund_amount,
        );

        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[fund_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        )).await.unwrap();

        // Initialize multisig
        let owner1 = Keypair::new();
        let owner2 = Keypair::new();
        let owners = vec![payer.pubkey(), owner1.pubkey(), owner2.pubkey()];
        let threshold = 2u64;

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
            }.try_to_vec().unwrap(),
        };

        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[initialize_multisig_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        )).await.unwrap();

        // Create transaction
        let recipient = Keypair::new();
        let transfer_amount = 1_000_000;

        let transfer_ix = solana_program::system_instruction::transfer(
            &vault_pubkey,
            &recipient.pubkey(),
            transfer_amount,
        );

        let transaction_accounts = vec![
            TransactionAccount {
                pubkey: vault_pubkey,
                is_signer: true,
                is_writable: true,
            },
            TransactionAccount {
                pubkey: recipient.pubkey(),
                is_signer: false,
                is_writable: true,
            },
            TransactionAccount {
                pubkey: system_program::id(),
                is_signer: false,
                is_writable: false,
            },
        ];

        let create_transaction_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::CreateMultiSigTransaction {
                program_id: system_program::id(),
                accounts: transaction_accounts,
                data: transfer_ix.data,
            }.try_to_vec().unwrap(),
        };

        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[create_transaction_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        )).await.unwrap();

        // Approve with owner1
        let approve_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(owner1.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::ApproveMultiSigTransaction {
                transaction_id: 0,
            }.try_to_vec().unwrap(),
        };

        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[approve_ix],
            Some(&payer.pubkey()),
            &[&payer, &owner1],
            recent_blockhash,
        )).await.unwrap();

        // Derive multisig signer PDA
        let (multisig_signer, _) = Pubkey::find_program_address(
            &[vault_pubkey.as_ref(), &[0]],
            &PROGRAM_ID,
        );

        // Execute transaction
        let execute_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(multisig_signer, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
                // Include the accounts needed for the transfer
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new(recipient.pubkey(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data: VaultInstruction::ExecuteMultiSigTransaction {
                transaction_id: 0,
            }.try_to_vec().unwrap(),
        };

        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[execute_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        )).await.unwrap();

        // Verify transaction was executed
        let vault_account = banks_client.get_account(vault_pubkey).await.unwrap().unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        let tx = &vault.multi_sig_transactions[0];
        assert!(tx.did_execute);

        // Verify transfer occurred
        let recipient_account = banks_client.get_account(recipient.pubkey()).await.unwrap().unwrap();
        assert_eq!(recipient_account.lamports, transfer_amount);
    }

    #[tokio::test]
    async fn test_multisig_threshold_validation() {
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;

        let vault_pubkey = create_vault(&mut banks_client, &payer, &recent_blockhash).await;

        // Initialize multisig with threshold 3
        let owner1 = Keypair::new();
        let owner2 = Keypair::new();
        let owner3 = Keypair::new();
        let owner4 = Keypair::new();
        let owners = vec![payer.pubkey(), owner1.pubkey(), owner2.pubkey(), owner3.pubkey(), owner4.pubkey()];
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
            }.try_to_vec().unwrap(),
        };

        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[initialize_multisig_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        )).await.unwrap();

        // Create transaction
        let recipient = Keypair::new();
        let transfer_ix = solana_program::system_instruction::transfer(
            &vault_pubkey,
            &recipient.pubkey(),
            1_000_000,
        );

        let transaction_accounts = vec![
            TransactionAccount {
                pubkey: vault_pubkey,
                is_signer: true,
                is_writable: true,
            },
            TransactionAccount {
                pubkey: recipient.pubkey(),
                is_signer: false,
                is_writable: true,
            },
        ];

        let create_transaction_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::CreateMultiSigTransaction {
                program_id: system_program::id(),
                accounts: transaction_accounts,
                data: transfer_ix.data,
            }.try_to_vec().unwrap(),
        };

        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[create_transaction_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        )).await.unwrap();

        // Approve with only 1 additional owner (total 2 approvals, but need 3)
        let approve_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(owner1.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::ApproveMultiSigTransaction {
                transaction_id: 0,
            }.try_to_vec().unwrap(),
        };

        banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[approve_ix],
            Some(&payer.pubkey()),
            &[&payer, &owner1],
            recent_blockhash,
        )).await.unwrap();

        // Try to execute with only 2 approvals (should fail)
        let (multisig_signer, _) = Pubkey::find_program_address(
            &[vault_pubkey.as_ref(), &[0]],
            &PROGRAM_ID,
        );

        let execute_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(vault_pubkey, false),
                AccountMeta::new_readonly(multisig_signer, false),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::sysvar::clock::id(), false),
            ],
            data: VaultInstruction::ExecuteMultiSigTransaction {
                transaction_id: 0,
            }.try_to_vec().unwrap(),
        };

        // Should fail due to insufficient approvals
        assert!(banks_client.process_transaction(Transaction::new_signed_with_payer(
            &[execute_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        )).await.is_err());

        // Verify transaction was not executed
        let vault_account = banks_client.get_account(vault_pubkey).await.unwrap().unwrap();
        let vault: Vault = Vault::try_from_slice(&vault_account.data).unwrap();

        let tx = &vault.multi_sig_transactions[0];
        assert!(!tx.did_execute);
    }
}
