# Vault Program CLI Tool

A comprehensive command-line interface for interacting with the Solana Vault Program on devnet.

## 🚀 Features

- **Program Deployment**: Deploy the vault program to devnet
- **Vault Management**: Initialize, pause, and unpause vaults
- **Token Operations**: Deposit and withdraw tokens
- **Account Management**: Create keypairs and check balances
- **Devnet Integration**: Full devnet testing capabilities
- **Environment Configuration**: Manage RPC URLs and settings via .env file

## 📋 Prerequisites

1. **Rust**: Install Rust and Cargo
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Solana CLI**: Install Solana CLI tools
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"
   ```

3. **Build Tools**: Install required build dependencies
   ```bash
   cargo install cargo-build-bpf
   ```

## 🛠️ Installation

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd vault_program
   ```

2. **Build the CLI**:
   ```bash
   cargo build --bin vault-cli
   ```

3. **Make test script executable**:
   ```bash
   chmod +x test_vault.sh
   ```

## 🔧 Environment Configuration

The CLI uses a `.env` file to manage configuration. Create a `.env` file in the project root:

```bash
# Solana RPC URLs
RPC_URL=https://api.devnet.solana.com
RPC_WS_URL=wss://api.devnet.solana.com

# Program ID
PROGRAM_ID=VAULT1111111111111111111111111111111111111111111111111111111111111111

# Default keypair path
DEFAULT_KEYPAIR_PATH=keypair.json

# Alternative networks (uncomment to use)
# Mainnet
# RPC_URL=https://api.mainnet-beta.solana.com
# RPC_WS_URL=wss://api.mainnet-beta.solana.com

# Testnet
# RPC_URL=https://api.testnet.solana.com
# RPC_WS_URL=wss://api.testnet.solana.com

# Local validator
# RPC_URL=http://127.0.0.1:8899
# RPC_WS_URL=ws://127.0.0.1:8900
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RPC_URL` | Solana RPC endpoint URL | `https://api.devnet.solana.com` |
| `RPC_WS_URL` | Solana WebSocket endpoint URL | `wss://api.devnet.solana.com` |
| `PROGRAM_ID` | Vault program ID | `VAULT1111111111111111111111111111111111111111111111111111111111111111` |
| `DEFAULT_KEYPAIR_PATH` | Default keypair file path | `keypair.json` |

## 📖 Usage

### Basic Commands

#### 1. Show Configuration
```bash
# Show current configuration
cargo run --bin vault-cli -- config

# Show configuration with secrets
cargo run --bin vault-cli -- config --show-secrets
```

#### 2. Create a Keypair
```bash
cargo run --bin vault-cli -- create-keypair --output-path my_keypair.json
```

#### 3. Check Account Balance
```bash
cargo run --bin vault-cli -- balance --account <ACCOUNT_ADDRESS>
```

#### 4. Get Vault Information
```bash
cargo run --bin vault-cli -- info --vault-account <VAULT_ADDRESS>
```

#### 5. Test Vault Functionality
```bash
cargo run --bin vault-cli -- test --vault-account <VAULT_ADDRESS>
```

### Advanced Options

#### Custom RPC URL (overrides .env)
```bash
cargo run --bin vault-cli -- --rpc-url https://api.mainnet-beta.solana.com <command>
```

#### Custom Keypair (overrides .env)
```bash
cargo run --bin vault-cli -- --keypair custom_keypair.json <command>
```

#### Using Different Networks
- **Devnet**: `https://api.devnet.solana.com` (default)
- **Testnet**: `https://api.testnet.solana.com`
- **Mainnet**: `https://api.mainnet-beta.solana.com`
- **Local**: `http://127.0.0.1:8899`

## 🧪 Testing

### Quick Configuration Test
```bash
# Test configuration loading
cargo run --bin vault-cli -- config

# Test with custom RPC URL
cargo run --bin vault-cli -- --rpc-url https://api.mainnet-beta.solana.com config
```

### Automated Test Suite

Run the comprehensive test suite:

```bash
./test_vault.sh
```

This script will:
1. ✅ Check prerequisites
2. ✅ Build the program
3. ✅ Create test keypair
4. ✅ Deploy to devnet
5. ✅ Test all vault functionality
6. ✅ Clean up test files

### Manual Testing

1. **Build and Deploy**:
   ```bash
   cargo build-bpf --target bpfel-unknown-unknown --release
   solana program deploy target/deploy/vault_program.so --keypair <keypair_path> --url <rpc_url>
   ```

2. **Test Vault Operations**:
   ```bash
   # Create keypair
   cargo run --bin vault-cli -- create-keypair --output-path test_keypair.json

   # Test vault functionality
   cargo run --bin vault-cli -- test --vault-account <VAULT_ADDRESS>
   ```

## 🔧 Configuration Management

### Priority Order
The CLI uses the following priority for configuration:

1. **Command Line Arguments** (highest priority)
2. **Environment Variables** (from .env file)
3. **Default Values** (lowest priority)

### Configuration Examples

#### Development Environment
```bash
# .env
RPC_URL=https://api.devnet.solana.com
RPC_WS_URL=wss://api.devnet.solana.com
PROGRAM_ID=VAULT1111111111111111111111111111111111111111111111111111111111111111
DEFAULT_KEYPAIR_PATH=dev_keypair.json
```

#### Production Environment
```bash
# .env
RPC_URL=https://api.mainnet-beta.solana.com
RPC_WS_URL=wss://api.mainnet-beta.solana.com
PROGRAM_ID=VAULT1111111111111111111111111111111111111111111111111111111111111111
DEFAULT_KEYPAIR_PATH=prod_keypair.json
```

#### Local Development
```bash
# .env
RPC_URL=http://127.0.0.1:8899
RPC_WS_URL=ws://127.0.0.1:8900
PROGRAM_ID=VAULT1111111111111111111111111111111111111111111111111111111111111111
DEFAULT_KEYPAIR_PATH=local_keypair.json
```

## 📊 Command Reference

### Global Options
- `--rpc-url <URL>`: RPC endpoint URL (overrides .env)
- `--keypair <PATH>`: Path to keypair file (overrides .env)

### Commands

#### `config`
Show current configuration
- `--show-secrets`: Display private keys (use with caution)

#### `create-keypair`
Create new keypair
- `--output-path <PATH>`: Output file path

#### `balance`
Check account balance
- `--account <ADDRESS>`: Account address

#### `info`
Get vault information
- `--vault-account <ADDRESS>`: Vault account address

#### `test`
Test vault functionality
- `--vault-account <ADDRESS>`: Vault account address

## 🐛 Troubleshooting

### Common Issues

1. **Environment File Not Found**:
   ```bash
   # Create .env file
   touch .env
   echo "RPC_URL=https://api.devnet.solana.com" >> .env
   ```

2. **Build Errors**:
   ```bash
   # Clean and rebuild
   cargo clean
   cargo build --bin vault-cli
   ```

3. **RPC Connection Issues**:
   ```bash
   # Check network connectivity
   curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' https://api.devnet.solana.com
   ```

4. **Insufficient SOL**:
   ```bash
   # Request airdrop
   solana airdrop 2 --keypair <keypair_path> --url <rpc_url>
   ```

5. **Configuration Issues**:
   ```bash
   # Check current configuration
   cargo run --bin vault-cli -- config
   ```

### Debug Mode

Enable verbose logging:
```bash
RUST_LOG=debug cargo run --bin vault-cli -- <command>
```

## 📈 Performance

### Optimization Tips

1. **Use Release Build**:
   ```bash
   cargo build --release --bin vault-cli
   ```

2. **Environment Caching**: The CLI caches environment variables for better performance

3. **Connection Pooling**: Reuse RPC connections for multiple operations

## 🔒 Security

### Best Practices

1. **Keypair Management**:
   - Store keypairs securely
   - Use environment variables for sensitive data
   - Never commit keypairs to version control
   - Use `--show-secrets` only when necessary

2. **Network Security**:
   - Use HTTPS for RPC connections
   - Verify program IDs before deployment
   - Test on devnet before mainnet

3. **Environment Security**:
   - Keep `.env` files secure
   - Use different configurations for different environments
   - Never commit `.env` files to version control

## 📞 Support

For issues and questions:
1. Check the troubleshooting section
2. Review the test suite output
3. Check Solana network status
4. Verify program deployment
5. Use the `config` command to verify settings

## 🎯 Next Steps

After successful testing:
1. Deploy to mainnet-beta
2. Implement additional DeFi integrations
3. Add more advanced vault features
4. Create web interface
5. Implement monitoring and analytics

---

**Happy Vaulting! 🚀**
