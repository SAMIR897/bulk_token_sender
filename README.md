# 📦 Bulk Token Sender

A Rust CLI tool that simulates batch ETH transfers to multiple addresses from a CSV file. Connects to a live Ethereum RPC for chain context.

## Features

- Reads recipient addresses and amounts from a CSV file
- Validates Ethereum addresses using `ethers-rs`
- Converts amounts to Wei for transaction simulation
- Shows current block number from the connected RPC

## Setup

```bash
cp .env.example .env
# Optionally set your RPC_URL (defaults to a public RPC)
```

## Usage

```bash
cargo run
```

The tool reads from `addresses.csv`. If the file doesn't exist, it creates a sample one automatically.

### CSV Format

```csv
address,amount
0x0000000000000000000000000000000000000000,0.1
0x1111111111111111111111111111111111111111,0.5
```

### Sample Output

```
Bulk Token Sender (Simulation Mode)
Current Block Number: 19432156
Simulating send of 0.1 ETH (100000000000000000 wei) to 0x0000...0000
Simulating send of 0.5 ETH (500000000000000000 wei) to 0x1111...1111
```

## Tech Stack

- `ethers` — Ethereum toolkit (address validation, unit conversion)
- `csv` — CSV file parsing
- `serde` — Deserialization
- `tokio` — Async runtime
