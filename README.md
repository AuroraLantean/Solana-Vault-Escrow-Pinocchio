# Solana-Pinocchio

## Instroduction to Pinocchio

<https://github.com/anza-xyz/pinocchio>

### Environment

Rust: 1.91.1 (ed61e7d7e 2025-11-07)
solana-cli: 3.0.12 or 2.3.13
BunJs:  1.3.3
PNPM: 10.24.0

## Setup a new Pinocchio project

```bash
cargo new program-name --lib --edition 2021
cd program-name
cargo add pinocchio pinocchio-system pinocchio-log pinocchio-pubkey shank
```

Add config in `Cargo.toml`

```toml
[lib]
crate-type = ["lib", "cdylib"]
```

### Make Program ID

```bash
solana-keygen new -o target/deploy/vault-keypair.json
solana address -k target/deploy/vault-keypair.json
```

Paste it into lib.rs > declare_id! macro.

### Build and Deploy the Program locally

```bash
cargo build-sbf
solana program deploy --program-id target/deploy/vault-keypair.json target/deploy/pinocchio_vault.so --url localhost 
```

### Make IDL via Shank for new instruction layout

```bash
cargo install shank-cli
shank idl -o idl
```

The idl folder in our project now contains pinocchio_vault.json which is our program's IDL.

### Make a client via @solana/kit and Codama

Codama takes the Shank IDL and emits a TypeScript client. The generated code includes instruction builders, account types, and small conveniences that keep your client code focused on composing transactions.

```bash
  pnpm install
  pnpm dlx codama init
  pnpm dlx codama run js
```

You'll see a clients/js/src/generated/ folder in our project with the program types our client code uses to send transactions to our program.


### Reference

<https://www.quicknode.com/guides/solana-development/pinocchio/how-to-build-and-deploy-a-solana-program-using-pinocchio>
