const {
  Connection,
  PublicKey,
  Keypair,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
} = require('@solana/web3.js');
const fs = require('fs');

// Program ID from deployment
const PROGRAM_ID = new PublicKey('DvMJg65xGz7W7xa1tP6LW2RP4TecJDb5oN2Qcvf7Qc63');

// Vault instruction enum (matching Rust enum)
const VaultInstruction = {
  Initialize: 0,
  Deposite: 1,
  Withdraw: 2,
  InitializeMultiSig: 3,
  ApproveTransaction: 4,
  ExecuteTransaction: 5,
  InitializeTimeLock: 6,
  InitializeGovernance: 7,
};

async function createVault() {
  // Connect to devnet
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  // Load vault keypair (this will be our authority)
  const keypairData = JSON.parse(fs.readFileSync('./vault-keypair.json', 'utf8'));
  const authorityKeypair = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log('Authority Public Key:', authorityKeypair.publicKey.toString());

  // Find PDA for vault
  const [vaultPDA, bump] = await PublicKey.findProgramAddress(
    [Buffer.from('vault'), authorityKeypair.publicKey.toBuffer()],
    PROGRAM_ID
  );

  console.log('Vault PDA:', vaultPDA.toString());
  console.log('Bump seed:', bump);

  // Check if vault PDA exists
  const accountInfo = await connection.getAccountInfo(vaultPDA);
  console.log('Vault PDA exists:', !!accountInfo);
  if (accountInfo) {
    console.log('Account balance:', accountInfo.lamports / LAMPORTS_PER_SOL, 'SOL');
    console.log('Account owner:', accountInfo.owner.toString());
  }

  // Create emergency admin keypair
  const emergencyAdminKeypair = Keypair.generate();
  console.log('Emergency Admin:', emergencyAdminKeypair.publicKey.toString());

  // Fund emergency admin
  await connection.confirmTransaction(
    await connection.requestAirdrop(emergencyAdminKeypair.publicKey, 0.01 * LAMPORTS_PER_SOL)
  );

  // Initialize vault instruction data
  const initializeData = Buffer.from([
    VaultInstruction.Initialize, // instruction type
    bump, // bump seed
  ]);

  const initializeIx = new TransactionInstruction({
    keys: [
      { pubkey: vaultPDA, isSigner: false, isWritable: true }, // vault_account
      { pubkey: authorityKeypair.publicKey, isSigner: true, isWritable: true }, // authority
      { pubkey: emergencyAdminKeypair.publicKey, isSigner: false, isWritable: false }, // emergency_admin
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system_program
      { pubkey: new PublicKey('SysvarRent111111111111111111111111111111111'), isSigner: false, isWritable: false }, // rent_sysvar
      { pubkey: new PublicKey('SysvarC1ock11111111111111111111111111111111'), isSigner: false, isWritable: false }, // clock_sysvar
    ],
    programId: PROGRAM_ID,
    data: initializeData,
  });

  // Create and send transaction
  const transaction = new Transaction().add(initializeIx);

  try {
    const signature = await sendAndConfirmTransaction(connection, transaction, [authorityKeypair]);
    console.log('✅ Vault initialized successfully!');
    console.log('Transaction signature:', signature);
    console.log('Vault PDA:', vaultPDA.toString());
    console.log('Authority:', authorityKeypair.publicKey.toString());
    console.log('Emergency Admin:', emergencyAdminKeypair.publicKey.toString());
  } catch (error) {
    console.error('❌ Error initializing vault:', error);
    console.error('Full error:', error);
  }
}

createVault();
