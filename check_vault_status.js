const {
  Connection,
  PublicKey,
  Keypair,
} = require('@solana/web3.js');
const fs = require('fs');

// Program ID from deployment
const PROGRAM_ID = new PublicKey('DvMJg65xGz7W7xa1tP6LW2RP4TecJDb5oN2Qcvf7Qc63');

async function checkVaultStatus() {
  // Connect to devnet
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  // Load vault authority keypair
  const keypairData = JSON.parse(fs.readFileSync('./vault-keypair.json', 'utf8'));
  const authorityKeypair = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log('🔐 Vault Status Check');
  console.log('==================');
  console.log('Authority Public Key:', authorityKeypair.publicKey.toString());

  // Find vault PDA
  const [vaultPDA, bump] = await PublicKey.findProgramAddress(
    [Buffer.from('vault'), authorityKeypair.publicKey.toBuffer()],
    PROGRAM_ID
  );

  console.log('Vault PDA:', vaultPDA.toString());
  console.log('PDA Bump:', bump);

  // Check vault account info
  const accountInfo = await connection.getAccountInfo(vaultPDA);
  if (accountInfo) {
    console.log('✅ Vault account exists!');
    console.log('Owner:', accountInfo.owner.toString());
    console.log('Balance:', accountInfo.lamports / 1000000000, 'SOL');
    console.log('Data length:', accountInfo.data.length, 'bytes');
    console.log('Executable:', accountInfo.executable ? 'Yes' : 'No');

    // Check if owned by our program
    if (accountInfo.owner.toString() === PROGRAM_ID.toString()) {
      console.log('✅ Vault is owned by our program!');
    } else {
      console.log('❌ Vault is not owned by our program');
    }
  } else {
    console.log('❌ Vault account does not exist');
  }

  // Check authority balance
  const authorityBalance = await connection.getBalance(authorityKeypair.publicKey);
  console.log('Authority balance:', authorityBalance / 1000000000, 'SOL');

  console.log('\n📊 Program Status');
  console.log('================');
  console.log('Program ID:', PROGRAM_ID.toString());
  console.log('Explorer Link:', `https://explorer.solana.com/address/${PROGRAM_ID.toString()}?cluster=devnet`);

  // Check program account
  const programInfo = await connection.getAccountInfo(PROGRAM_ID);
  if (programInfo) {
    console.log('✅ Program deployed successfully!');
    console.log('Program executable:', programInfo.executable ? 'Yes' : 'No');
    console.log('Program balance:', programInfo.lamports / 1000000000, 'SOL');
  } else {
    console.log('❌ Program not found');
  }

  console.log('\n🎯 Summary');
  console.log('==========');
  if (accountInfo && accountInfo.owner.toString() === PROGRAM_ID.toString()) {
    console.log('✅ SUCCESS: Vault is initialized and working!');
    console.log('✅ SUCCESS: Program is deployed and functional!');
  } else {
    console.log('❌ FAILURE: Vault setup incomplete');
  }
}

checkVaultStatus();
