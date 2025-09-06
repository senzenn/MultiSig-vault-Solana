# 🌍 Environment Configuration Guide

## Overview

The Vault Program CLI uses environment variables to manage configuration across different networks and environments. This guide explains how to set up and use the `.env` file for optimal development and deployment.

---

## 📁 File Structure

```
vault_program/
├── .env                    # Environment configuration
├── src/
│   ├── cli.rs             # CLI with env support
│   └── ...
├── Cargo.toml             # Dependencies including dotenv
└── ...
```

---

## 🔧 Environment Variables

### Core Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `RPC_URL` | Solana RPC endpoint URL | `https://api.devnet.solana.com` | ✅ |
| `RPC_WS_URL` | Solana WebSocket endpoint URL | `wss://api.devnet.solana.com` | ❌ |
| `PROGRAM_ID` | Vault program ID | `VAULT1111111111111111111111111111111111111111111111111111111111111111` | ✅ |
| `DEFAULT_KEYPAIR_PATH` | Default keypair file path | `keypair.json` | ❌ |

### Network-Specific URLs

#### Devnet (Default)
```bash
RPC_URL=https://api.devnet.solana.com
RPC_WS_URL=wss://api.devnet.solana.com
```

#### Testnet
```bash
RPC_URL=https://api.testnet.solana.com
RPC_WS_URL=wss://api.testnet.solana.com
```

#### Mainnet
```bash
RPC_URL=https://api.mainnet-beta.solana.com
RPC_WS_URL=wss://api.mainnet-beta.solana.com
```

#### Local Validator
```bash
RPC_URL=http://127.0.0.1:8899
RPC_WS_URL=ws://127.0.0.1:8900
```

---

## 🚀 Quick Setup

### 1. Create Environment File

```bash
# Create .env file
touch .env

# Add basic configuration
echo "RPC_URL=https://api.devnet.solana.com" >> .env
echo "RPC_WS_URL=wss://api.devnet.solana.com" >> .env
echo "PROGRAM_ID=VAULT1111111111111111111111111111111111111111111111111111111111111111" >> .env
echo "DEFAULT_KEYPAIR_PATH=keypair.json" >> .env
```

### 2. Verify Configuration

```bash
# Check current configuration
cargo run --bin vault-cli -- config
```

### 3. Test Environment Loading

```bash
# Test with custom RPC URL
cargo run --bin vault-cli -- --rpc-url https://api.mainnet-beta.solana.com config
```

---

## 🔄 Environment Switching

### Development Environment

```bash
# .env
RPC_URL=https://api.devnet.solana.com
RPC_WS_URL=wss://api.devnet.solana.com
PROGRAM_ID=VAULT1111111111111111111111111111111111111111111111111111111111111111
DEFAULT_KEYPAIR_PATH=dev_keypair.json
```

### Production Environment

```bash
# .env
RPC_URL=https://api.mainnet-beta.solana.com
RPC_WS_URL=wss://api.mainnet-beta.solana.com
PROGRAM_ID=VAULT1111111111111111111111111111111111111111111111111111111111111111
DEFAULT_KEYPAIR_PATH=prod_keypair.json
```

### Local Development

```bash
# .env
RPC_URL=http://127.0.0.1:8899
RPC_WS_URL=ws://127.0.0.1:8900
PROGRAM_ID=VAULT1111111111111111111111111111111111111111111111111111111111111111
DEFAULT_KEYPAIR_PATH=local_keypair.json
```

---

## 🎯 Configuration Priority

The CLI uses the following priority order for configuration:

1. **Command Line Arguments** (highest priority)
   ```bash
   cargo run --bin vault-cli -- --rpc-url https://api.mainnet-beta.solana.com config
   ```

2. **Environment Variables** (from .env file)
   ```bash
   # .env
   RPC_URL=https://api.devnet.solana.com
   ```

3. **Default Values** (lowest priority)
   ```rust
   // Hardcoded defaults in CLI
   "https://api.devnet.solana.com"
   ```

---

## 🔒 Security Best Practices

### 1. Environment File Security

```bash
# Add .env to .gitignore
echo ".env" >> .gitignore
echo "*.keypair" >> .gitignore
echo "keypair.json" >> .gitignore
```

### 2. Keypair Management

```bash
# Use environment variable for keypair path
DEFAULT_KEYPAIR_PATH=/secure/path/to/keypair.json

# Or use command line override
cargo run --bin vault-cli -- --keypair /secure/path/to/keypair.json config
```

### 3. Network Security

```bash
# Always verify network before use
cargo run --bin vault-cli -- config

# Check network connectivity
curl -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' \
  https://api.devnet.solana.com
```

---

## 🧪 Testing Configuration

### 1. Configuration Validation

```bash
# Test configuration loading
cargo run --bin vault-cli -- config

# Test with secrets (development only)
cargo run --bin vault-cli -- config --show-secrets
```

### 2. Network Connectivity Test

```bash
# Test RPC connection
cargo run --bin vault-cli -- test --vault-account <VAULT_ADDRESS>
```

### 3. Environment Override Test

```bash
# Test command line override
cargo run --bin vault-cli -- --rpc-url https://api.mainnet-beta.solana.com config
```

---

## 🔧 Advanced Configuration

### Multiple Environment Files

```bash
# Development
cp .env .env.dev

# Production
cp .env .env.prod

# Load specific environment
dotenv -f .env.dev cargo run --bin vault-cli -- config
```

### Environment-Specific Scripts

```bash
#!/bin/bash
# scripts/dev.sh
export RPC_URL=https://api.devnet.solana.com
export RPC_WS_URL=wss://api.devnet.solana.com
cargo run --bin vault-cli -- "$@"
```

```bash
#!/bin/bash
# scripts/prod.sh
export RPC_URL=https://api.mainnet-beta.solana.com
export RPC_WS_URL=wss://api.mainnet-beta.solana.com
cargo run --bin vault-cli -- "$@"
```

### CI/CD Integration

```yaml
# .github/workflows/deploy.yml
name: Deploy to Devnet
on:
  push:
    branches: [main]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Setup Environment
        run: |
          echo "RPC_URL=https://api.devnet.solana.com" >> .env
          echo "PROGRAM_ID=${{ secrets.PROGRAM_ID }}" >> .env
      - name: Deploy
        run: cargo run --bin vault-cli -- deploy
```

---

## 🐛 Troubleshooting

### Common Issues

#### 1. Environment File Not Found
```bash
# Error: Environment variables not loading
# Solution: Create .env file
touch .env
echo "RPC_URL=https://api.devnet.solana.com" >> .env
```

#### 2. Invalid RPC URL
```bash
# Error: Connection failed
# Solution: Verify RPC URL
curl -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' \
  https://api.devnet.solana.com
```

#### 3. Keypair Not Found
```bash
# Error: Keypair file not found
# Solution: Create keypair or update path
cargo run --bin vault-cli -- create-keypair --output-path keypair.json
```

#### 4. Configuration Priority Issues
```bash
# Error: Wrong network being used
# Solution: Check priority order
cargo run --bin vault-cli -- config
```

### Debug Commands

```bash
# Enable debug logging
RUST_LOG=debug cargo run --bin vault-cli -- config

# Check environment variables
env | grep RPC

# Verify .env file content
cat .env
```

---

## 📋 Environment Checklist

### ✅ Setup Checklist

- [ ] `.env` file created
- [ ] `RPC_URL` configured
- [ ] `PROGRAM_ID` set
- [ ] `DEFAULT_KEYPAIR_PATH` configured (optional)
- [ ] `.env` added to `.gitignore`
- [ ] Configuration tested with `config` command
- [ ] Network connectivity verified
- [ ] Keypair created and accessible

### ✅ Security Checklist

- [ ] `.env` file not committed to version control
- [ ] Keypair stored securely
- [ ] Network URLs verified
- [ ] No secrets in logs
- [ ] Environment-specific configurations used

---

## 🎉 Success Indicators

### Configuration Working
```bash
$ cargo run --bin vault-cli -- config
🔧 Current Configuration
=======================
RPC URL: https://api.devnet.solana.com
WebSocket URL: wss://api.devnet.solana.com
Program ID: VAULT1111111111111111111111111111111111111111111111111111111111111111
Public Key: GEfcYtkZ22yNZZq26yXkbq6QF4QCzYygpjJiUDm1r5v
Private Key: [HIDDEN] (use --show-secrets to view)

📋 Environment Variables:
  RPC_URL: https://api.devnet.solana.com
  RPC_WS_URL: wss://api.devnet.solana.com
  PROGRAM_ID: VAULT1111111111111111111111111111111111111111111111111111111111111111
  DEFAULT_KEYPAIR_PATH: keypair.json
```

### Network Connectivity
```bash
$ cargo run --bin vault-cli -- test --vault-account <VAULT_ADDRESS>
🧪 Testing Vault Functionality
==============================
1. Checking vault account existence...
   ✅ Vault account exists
2. Checking keypair balance...
   💰 Balance: 2.5 SOL
3. Checking program ID...
   🆔 Program ID: VAULT1111111111111111111111111111111111111111111111111111111111111111
4. Testing network connectivity...
   ✅ Network connected
5. Checking environment configuration...
   🌍 RPC URL: https://api.devnet.solana.com
   🔌 WebSocket URL: wss://api.devnet.solana.com
   🆔 Program ID: VAULT1111111111111111111111111111111111111111111111111111111111111111
```

---

## 🚀 Next Steps

1. **Deploy to Devnet**: Use devnet configuration
2. **Test on Testnet**: Switch to testnet configuration
3. **Deploy to Mainnet**: Use mainnet configuration
4. **Monitor Performance**: Use WebSocket for real-time updates
5. **Scale Infrastructure**: Use multiple RPC endpoints

---

*Environment configuration is the foundation of reliable Solana development! 🌟*
