# Solana Development Environment Setup and Initialization Guide

This repository provides a step-by-step guide to installing the Solana Command Line Interface (CLI), Rust, and the Anchor framework on Windows, macOS, and Linux systems. It includes instructions for resolving path environment variables and initializing a local workspace.

---

## 1. Install Rust

Solana programs are written using the Rust programming language. Follow the instructions below based on your operating system.

### macOS and Linux
Install Rust using the official installation script:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://rustup.rs | sh
```
Select the default installation option (1) when prompted. Then, configure your current shell environment:
```bash
source ~/.profile
source ~/.cargo/env
```

### Windows
1. Download and run the **rustup-init.exe** installer from the official [Rust Language Website](https://rustup.rs).
2. If prompted, install the Microsoft Visual C++ Build Tools before proceeding.
3. Select the default installation option (1) and complete the setup.
4. Restart your terminal or Command Prompt.

### Verify Rust Installation
Ensure the compiler is accessible:
```bash
rustc --version
```

---

## 2. Install Solana CLI

### macOS and Linux
Download and install the Solana tool suite:
```bash
sh -c "\$(curl -sSfL https://solana.com)"
```

#### Environment Path Configuration (macOS/Linux)
If the `solana` command is not recognized, update your environment variables. 

For **Ubuntu/Linux (Bash)**:
```bash
echo 'export PATH="\(HOME/.local/share/solana/install/active_release/bin:\)PATH"' >> ~/.bashrc
source ~/.bashrc
```

For **macOS (Zsh)**:
```bash
echo 'export PATH="\(HOME/.local/share/solana/install/active_release/bin:\)PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Windows
1. Open PowerShell as an Administrator and execute the installer script:
```powershell
cmd /c "curl https://solana.com --output solana-install.txt && solana-install.txt"
```
2. Close your current PowerShell window and open a new one to apply the environment changes automatically.

### Verify Solana Installation
Ensure the system accurately recognizes the software version:
```bash
solana --version
```

---

## 3. Install Anchor CLI

Anchor is the framework utilized for structuring and deploying secure Solana programs. 

### macOS and Linux
Install the Anchor CLI directly through Cargo:
```bash
cargo install anchor-cli --locked
```
*Note: Linux users may need to install prerequisite build tools using `sudo apt install build-essential pkg-config libssl-dev libudev-dev` before running this command.*

### Windows
Windows native support for Anchor is experimental. It is highly recommended to use the Windows Subsystem for Linux (WSL) and follow the Linux instructions. If installing natively on Windows, execute:
```powershell
cargo install anchor-cli --locked
```

### Verify Anchor Installation
```bash
anchor --version
```

---

## 4. Configure Local Solana Environment

Before generating program code, configure your command-line workspace variables and establish local cryptographic keypairs.

### 1. Set Cluster Network to Devnet
```bash
solana config set --url https://solana.com
```

### 2. Generate a Local Keypair (Development Wallet)
```bash
solana-keygen new
```
Press Enter to bypass the passphrase option for local testing environments.

### 3. Request Devnet Funds (Airdrop)
Request testnet tokens to pay for transaction and deployment fees:
```bash
solana airdrop 2
```

Confirm the account allocation balance:
```bash
solana balance
```

---

## 5. Initialize a New Project

The environment is now ready to generate, build, and test an Anchor workspace.

### 1. Initialize Project Workspace
```bash
anchor init hello-solana
cd hello-solana
```

### 2. Compile the Program
```bash
anchor build
```

### 3. Run Automated Tests
```bash
anchor test
```
A successful execution of the test suite validates that the development stack is fully operational across your system.
