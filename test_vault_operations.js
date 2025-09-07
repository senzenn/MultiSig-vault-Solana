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

async function testVaultOperations() {
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

  // Check current balances
  const authorityBalance = await connection.getBalance(authorityKeypair.publicKey);
  const vaultBalance = await connection.getBalance(vaultPDA);

  console.log('Authority balance:', authorityBalance / LAMPORTS_PER_SOL, 'SOL');
  console.log('Vault balance:', vaultBalance / LAMPORTS_PER_SOL, 'SOL');

  // ===== TEST 1: Deposit SOL =====
  console.log('\n🔧 Testing: Deposit SOL');

  const depositAmount = 0.05 * LAMPORTS_PER_SOL; // 0.05 SOL

  const depositData = Buffer.concat([
    Buffer.from([VaultInstruction.Deposite]), // instruction type
    new Uint8Array(new BigUint64Array([BigInt(depositAmount)]).buffer), // amount as u64
  ]);

  const depositIx = new TransactionInstruction({
    keys: [
      { pubkey: vaultPDA, isSigner: false, isWritable: true }, // vault_account
      { pubkey: authorityKeypair.publicKey, isSigner: true, isWritable: true }, // depositor
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system_program
    ],
    programId: PROGRAM_ID,
    data: depositData,
  });

  try {
    const depositTx = new Transaction().add(depositIx);
    const depositSignature = await sendAndConfirmTransaction(connection, depositTx, [authorityKeypair]);
    console.log('✅ Deposit successful!');
    console.log('Transaction signature:', depositSignature);

    // Check balances after deposit
    const newAuthorityBalance = await connection.getBalance(authorityKeypair.publicKey);
    const newVaultBalance = await connection.getBalance(vaultPDA);

    console.log('Authority balance after deposit:', newAuthorityBalance / LAMPORTS_PER_SOL, 'SOL');
    console.log('Vault balance after deposit:', newVaultBalance / LAMPORTS_PER_SOL, 'SOL');

  } catch (error) {
    console.error('❌ Deposit failed:', error.message);
  }

  // ===== TEST 2: Withdraw SOL =====
  console.log('\n💰 Testing: Withdraw SOL');

  const withdrawAmount = 0.02 * LAMPORTS_PER_SOL; // 0.02 SOL

  const withdrawData = Buffer.concat([
    Buffer.from([VaultInstruction.Withdraw]), // instruction type
    new Uint8Array(new BigUint64Array([BigInt(withdrawAmount)]).buffer), // amount as u64
  ]);

  const withdrawIx = new TransactionInstruction({
    keys: [
      { pubkey: vaultPDA, isSigner: false, isWritable: true }, // vault_account
      { pubkey: authorityKeypair.publicKey, isSigner: true, isWritable: true }, // withdrawer
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system_program
    ],
    programId: PROGRAM_ID,
    data: withdrawData,
  });

  try {
    const withdrawTx = new Transaction().add(withdrawIx);
    const withdrawSignature = await sendAndConfirmTransaction(connection, withdrawTx, [authorityKeypair]);
    console.log('✅ Withdrawal successful!');
    console.log('Transaction signature:', withdrawSignature);

    // Check final balances
    const finalAuthorityBalance = await connection.getBalance(authorityKeypair.publicKey);
    const finalVaultBalance = await connection.getBalance(vaultPDA);

    console.log('Final authority balance:', finalAuthorityBalance / LAMPORTS_PER_SOL, 'SOL');
    console.log('Final vault balance:', finalVaultBalance / LAMPORTS_PER_SOL, 'SOL');

  } catch (error) {
    console.error('❌ Withdrawal failed:', error.message);
  }

  console.log('\n🎉 Vault operations test completed!');
}

testVaultOperations();
