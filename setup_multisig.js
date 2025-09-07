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

async function setupMultisig() {
  // Connect to devnet
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  // Load vault authority keypair
  const keypairData = JSON.parse(fs.readFileSync('./vault-keypair.json', 'utf8'));
  const authorityKeypair = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log('Authority Public Key:', authorityKeypair.publicKey.toString());

  // Find vault PDA
  const [vaultPDA, bump] = await PublicKey.findProgramAddress(
    [Buffer.from('vault'), authorityKeypair.publicKey.toBuffer()],
    PROGRAM_ID
  );

  console.log('Vault PDA:', vaultPDA.toString());

  // Generate multisig owners
  const owner1 = Keypair.generate();
  const owner2 = Keypair.generate();
  const owner3 = Keypair.generate();

  console.log('Multisig Owner 1:', owner1.publicKey.toString());
  console.log('Multisig Owner 2:', owner2.publicKey.toString());
  console.log('Multisig Owner 3:', owner3.publicKey.toString());

  // Check owner account balances (skip funding to avoid rate limits)
  console.log('Checking multisig owner balances...');
  const owner1Balance = await connection.getBalance(owner1.publicKey);
  const owner2Balance = await connection.getBalance(owner2.publicKey);
  const owner3Balance = await connection.getBalance(owner3.publicKey);

  console.log('Owner 1 balance:', owner1Balance / LAMPORTS_PER_SOL, 'SOL');
  console.log('Owner 2 balance:', owner2Balance / LAMPORTS_PER_SOL, 'SOL');
  console.log('Owner 3 balance:', owner3Balance / LAMPORTS_PER_SOL, 'SOL');

  // Multisig configuration
  const owners = [owner1.publicKey, owner2.publicKey, owner3.publicKey];
  const threshold = 2; // Require 2 out of 3 signatures
  const nonce = 1; // Unique nonce for this multisig

  console.log(`Setting up multisig with ${owners.length} owners, threshold: ${threshold}`);

  // Serialize owners array for instruction data
  let ownersData = Buffer.alloc(0);
  owners.forEach(owner => {
    ownersData = Buffer.concat([ownersData, owner.toBuffer()]);
  });

  // Create InitializeMultiSig instruction data
  const multisigData = Buffer.concat([
    Buffer.from([VaultInstruction.InitializeMultiSig]), // instruction type
    Buffer.from([owners.length]), // number of owners
    ownersData, // owners array
    new Uint8Array(new BigUint64Array([BigInt(threshold)]).buffer), // threshold as u64
    Buffer.from([nonce]), // nonce
  ]);

  const initializeMultisigIx = new TransactionInstruction({
    keys: [
      { pubkey: vaultPDA, isSigner: false, isWritable: true }, // vault_account
      { pubkey: authorityKeypair.publicKey, isSigner: true, isWritable: false }, // initializer
      { pubkey: new PublicKey('SysvarC1ock11111111111111111111111111111111'), isSigner: false, isWritable: false }, // clock_sysvar
    ],
    programId: PROGRAM_ID,
    data: multisigData,
  });

  // Create and send transaction
  const transaction = new Transaction().add(initializeMultisigIx);

  try {
    const signature = await sendAndConfirmTransaction(connection, transaction, [authorityKeypair]);
    console.log('✅ Multisig initialized successfully!');
    console.log('Transaction signature:', signature);
    console.log('Vault with Multisig:', vaultPDA.toString());
    console.log('Threshold:', threshold, 'out of', owners.length, 'owners');

    // Save multisig configuration for later use
    const multisigConfig = {
      vaultPDA: vaultPDA.toString(),
      owners: owners.map(owner => owner.publicKey.toString()),
      threshold,
      nonce,
      ownerKeypairs: [owner1, owner2, owner3].map(kp => ({
        publicKey: kp.publicKey.toString(),
        secretKey: Array.from(kp.secretKey)
      }))
    };

    fs.writeFileSync('./multisig_config.json', JSON.stringify(multisigConfig, null, 2));
    console.log('Multisig configuration saved to multisig_config.json');

  } catch (error) {
    console.error('❌ Error initializing multisig:', error);
    console.error('Full error:', error);
  }
}

setupMultisig();
