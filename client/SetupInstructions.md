
# Requirements

This section outlines the prerequisites and steps needed to set up the environment for running Saturn Titan Indexer.

## Install rust

Rust is a programming language required for building and running certain tools in this setup, like the Saturn Titan Indexer. 

Follow these steps to install it. </br>Run the following command to download and install rustup, which will also install Rust and Cargo:


***Command:***

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Source the Rust Environment:

After installation, source the environment script to make cargo available in your current shell session

***Command:***
```bash
source $HOME/.cargo/env
```

Verify Installation:

***Command:***
```bash
rustc --version
cargo --version
```

## Bitcoin Node 27.0

This section guides you through setting up Bitcoin Core version 27.0, which will run a local Bitcoin node for testing or development purposes.

### 1. Download Bitcoin Core 27.0

Visit the official Bitcoin Core website to download the appropriate version for your operating system.

* Go to https://bitcoincore.org/bin/bitcoin-core-27.0/

### Choose Your File:

* For Linux, download a .tar.gz file:

    * bitcoin-27.0-x86_64-linux-gnu.tar.gz (48.8 MB) - For 64-bit Intel/AMD systems (most common desktops/laptops).

* For Windows, download a .zip file:

    * bitcoin-27.0-win64.zip - For 64-bit Windows systems (most modern PCs).

* For macOS (Apple), download a .tar.gz file (command-line version):

    * bitcoin-27.0-x86_64-apple-darwin.tar.gz (for Intel-based Macs)

    * bitcoin-27.0-arm64-apple-darwin.tar.gz (for Apple Silicon, e.g., M1/M2, if available)


### 2. Extract

Unpack the downloaded file to access the Bitcoin Core binaries.


***Command:***
```bash
tar -xzf ~/Downloads/bitcoin-27.0-arm64-apple-darwin.tar.gz
```

### 3. Verify Installation

Confirm that Bitcoin Core is installed correctly by checking its version.

***Command:***

    bitcoin-27.0/bin/bitcoind --version

***Output:***

    Bitcoin Core version v27.0.0
    Copyright (C) 2009-2024 The Bitcoin Core developers

    Please contribute if you find Bitcoin Core useful. Visit
    <https://bitcoincore.org/> for further information about the software.
    The source code is available from <https://github.com/bitcoin/bitcoin>.

    This is experimental software.
    Distributed under the MIT software license, see the accompanying file COPYING
    or <https://opensource.org/licenses/MIT>


### 4. Create the Bitcoin Data Directory

Set up a directory to store Bitcoin blockchain data and configuration files.

***Command:***
```bash
mkdir -p ~/Library/Application\ Support/Bitcoin
```

### 5. Create and Configure bitcoin.conf for Regtest

The bitcoin.conf file lets you specify settings for Bitcoin Core, including running in regtest mode.

***Command:***
```bash
cd ~/Library/Application\ Support/Bitcoin

nano bitcoin.conf
```

Add the Regtest Configuration

```bash
regtest=1
server=1
[regtest]
rpcuser=<USERNAME>
rpcpassword=<PASSWORD>
rpcport=18444
daemon=1
```

Ensure the file is only readable by your user

***Command:***
```bash
chmod 600 bitcoin.conf
```

### 6. Bitcoin node

Start and manage your Bitcoin node with these commands.

***Command:***
```bash
bitcoin-27.0/bin/bitcoind
```
#### Ensure Bitcoin node is running:

To verify that your Bitcoin Core node (version 27.0) is running, you can use the ps aux | grep bitcoind command. This checks for active Bitcoin processes.

***Command:***
```bash
ps aux | grep bitcoind
```
***Output:***

    <USER>       6563   0.0  0.0 409533904   3056   ??  Ss   11:23PM   0:18.66 /Users/<USER>/Documents/btc/bitcoin-27.0/bin/bitcoind
    <USER>      20431   0.0  0.0 408636096   1488 s000  S+   12:52PM   0:00.00 grep bitcoind

#### Check status of Bitcoin node

Get detailed information about the blockchain state.

***Command:***
```bash
bitcoin-27.0/bin/bitcoin-cli getblockchaininfo
```
***Output:***

    {
        "chain": "regtest",
        "blocks": 101,
        "headers": 101,
        "bestblockhash": "12edbfb555116b3eacfcf1cbb9f40c9bc8149d26cad1874df204b981f0e07356",
        "difficulty": 4.656542373906925e-10,
        "time": 1743203042,
        "mediantime": 1743203041,
        "verificationprogress": 1,
        "initialblockdownload": false,
        "chainwork": "00000000000000000000000000000000000000000000000000000000000000cc",
        "size_on_disk": 30375,
        "pruned": false,
        "warnings": ""
    }

## Wallets:

This section covers creating and managing wallets within Bitcoin Core for testing transactions.

### Create wallet

Generate a new wallet to store your test Bitcoin.

***Command:***
```bash
bitcoin-27.0/bin/bitcoin-cli createwallet "testwallet"
```
***Output:***

    {
        "name": "testwallet"
    }

### Get wallet address

Obtain a new address for receiving funds in your wallet.

***Command:***
```bash
bitcoin-27.0/bin/bitcoin-cli -rpcwallet="testwallet" getnewaddress
```
***Output:***

    bcrt1q8l6qw0w.......

### Generate block

Create blocks to mine test Bitcoin to your wallet address.

***Command:***
```bash
bitcoin-27.0/bin/bitcoin-cli -rpcwallet="testwallet" generatetoaddress 102 "<YOUR_ADDRESS>"
```
***Output:***

    [
        "05a40cca154d1556455456a39412189b38375e711c18f18a786680455fcc3871",
        "694d841e220007eb6dba3b3e3163f3ac95e5ea66c9db5dd4e6c80a664d182d01",
        "17379174882fd1e0ad84aed1f89905100e3a60fb375942c26a27a6efcd2a9629",
        "46d170c29b935601b430ee6b6b1f74c68c5c3dd9bc189fc66c9d375312847f61",
        "54e8c81358a95a5dab402c6f1f1bcd95f89090711d8cbc56ed5cd41498ef0f6e",
        "1cb1db9de585759684aa2fbe0220f87d1fa0d054bc3f49bf559636f7916c1ee6",
        "6c67e8f3bb8c8aa639e7b6aa3fb9ad3009ab7e8b9200b2d9d80b08d88b7819b2",
        ...
    ]


### If the Wallet Exists but Isn’t Loaded

Load an existing wallet if it’s not currently active.

***Command:***
```bash
bitcoin-27.0/bin/bitcoin-cli loadwallet "testwallet"
```
***Output:***

    {
        "name": "testwallet"
    }

### Verify vallet is there

Check the list of loaded wallets to ensure yours is present.

***Command:***
```bash
bitcoin-27.0/bin/bitcoin-cli listwallets
```
***Output:***

    {
        "name": "testwallet"
    }


# Setting up Saturn Titan Indexer

### Git clone Saturn Titan Indexer

***Command:***
```bash
git clone https://github.com/SaturnBTC/Titan.git 
```
### Create data dir

***Command:***
```bash
mkdir -p ~/titan-indexer
```
No need to create .env file in titan indexer project


### Run like this: (Just cli flags)

***Command:***
```bash
cd titan

./target/release/titan --bitcoin-rpc-url http://localhost:18444 --bitcoin-rpc-username <USERNAME> --bitcoin-rpc-password <PASSWORD> --chain regtest --index-addresses --index-bitcoin-transactions --enable-tcp-subscriptions --data-dir ~/titan-indexer
```
***Output:***

    2025-03-29T13:37:48.091391Z  INFO titan::subscription::dispatcher: event_dispatcher started
    2025-03-29T13:37:48.091367Z  INFO titan::subscription::spawn: Spawned subscription tasks (dispatcher + cleanup).
    2025-03-29T13:37:48.091569Z  INFO titan::subscription::tcp_subscription: TCP Subscription Server listening on 127.0.0.1:8080
    2025-03-29T13:37:48.093975Z  INFO titan: Spawned background threads
    2025-03-29T13:37:48.099776Z  INFO titan::server::server: Listening on http://0.0.0.0:3030
    2025-03-29T13:37:53.098638Z  INFO titan::index::metrics: Average Latency for batch_update_script_pubkeys_for_block: 0.016 ms
    2025-03-29T13:37:53.098720Z  INFO titan::index::metrics: Average Latency for index_mempool: 0.535 ms
    2025-03-29T13:37:53.098730Z  INFO titan::index::metrics: Average Latency for notify_tx_updates: 0.007 ms
    2025-03-29T13:37:58.101446Z  INFO titan::index::metrics: Average Latency for batch_update_script_pubkeys_for_block: 0.010 ms
    2025-03-29T13:37:58.101523Z  INFO titan::index::metrics: Average Latency for index_mempool: 0.472 ms
    2025-03-29T13:37:58.101531Z  INFO titan::index::metrics: Average Latency for notify_tx_updates: 0.006 ms
    2025-03-29T13:38:03.106764Z  INFO titan::index::metrics: Average Latency for batch_update_script_pubkeys_for_block: 0.009 ms
    2025-03-29T13:38:03.106836Z  INFO titan::index::metrics: Average Latency for index_mempool: 0.470 ms
    2025-03-29T13:38:03.106851Z  INFO titan::index::metrics: Average Latency for notify_tx_updates: 0.006 ms
    2025-03-29T13:38:08.109888Z  INFO titan::index::metrics: Average Latency for batch_update_script_pubkeys_for_block: 0.008 ms
    2025-03-29T13:38:08.109967Z  INFO titan::index::metrics: Average Latency for index_mempool: 0.458 ms
    2025-03-29T13:38:08.109976Z  INFO titan::index::metrics: Average Latency for notify_tx_updates: 0.005 ms
    ....


# Create new Rust project

## Run the Cargo New Command

Use the following command to create a new Rust project named my_project (replace my_project with your desired project name):

***Command:***
```bash
cargo new my_project
```
Inside Cargo.toml insert titan-client = "0.1.31"

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2025"

[dependencies]
titan-client = "0.1.31"
```

In main.rs insert this:

```rust
use titan_client::TitanApiBlocking;
use titan_client::TitanBlockingClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a blocking client.
    let client = TitanBlockingClient::new("http://localhost:3030");

    // Retrieve the indexer status.
    let status = client.get_status()?;
    println!("Status: {:?}", status);

    // Retrieve the current block tip.
    let tip = client.get_tip()?;
    println!("Block Tip: {:?}", tip);

    // Retrieve address data.
    let address_data = client.get_address("<YOUR ADDRESS>")?;
    println!("Address Data: {:?}", address_data);

    Ok(())
}
```

The Titan Indexer output shows the TCP Subscription server listening on 127.0.0.1:8080, while the HTTP server listens on http://0.0.0.0:3030.</br> 
To ensure your project functions correctly, connect the client to a valid address. In the example below, HTTP is used, which is why the client is initialized as:

### Run the project

***Command:***
```bash
cargo run
```
***Output:***

    Compiling btc-indexer v0.1.0 (/Users/<USER>/Documents/btc/rust-bitcoin-indexer/btc-indexer)
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.31s
        Running `target/debug/btc-indexer`
    Status: Status { block_tip: BlockTip { height: 101, hash: "04eae7346938a602cca0319b0a458bd2cebd8a97b129e52bd9fc265318cc8b08" }, runes_count: 0, mempool_tx_count: 0 }
    Block Tip: BlockTip { height: 101, 
    hash: "04eae7346938a602cca0319b0a458bd2cebd8a97b129e52bd9fc265318cc8b08" }
    Address Data: AddressData { value: 505000000000, runes: [], outputs: [AddressTxOut { txid: a173ea6c5b916609d4fea1f461407030237697b4d6dce9a91d4d294be7329e7e, vout: 0, value: 5000000000, runes: [], risky_runes: [], spent: Unspent, status: TransactionStatus { confirmed: true, block_height: Some(38), block_hash: Some(3dbfd76f44e61a67c7f78b5ef29ce5c5315e2d90a14ac4dfa2f5838ab1d36e3b) } }, AddressTxOut { txid: 7bfccb6caec9470a01eeeb541d5907be70013cec840e92287c6c5342954d168c, vout: 0, value: 5000000000, 
    runes: [], risky_runes: [], spent: Unspent, status: TransactionStatus { confirmed: true, block_height: Some(7), block_hash: Some(4375b37201052c6b33d790a2af8b09e6765d8825770823d56c9deff9bc4b41b5) } }, AddressTxOut { txid: 83f99568eab66ed1d8141e92c32b74a681e45dc7f8910b3b806091b18dc274c6, vout: 0, value: 5000000000, runes: [], risky_runes: [],  ...}...