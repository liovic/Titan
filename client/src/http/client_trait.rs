use crate::Error;
use async_trait::async_trait;
use bitcoin::Txid;
use reqwest::header::HeaderMap;
use titan_types::{
    query, AddressData, Block, BlockTip, Pagination, PaginationResponse, RuneResponse, Status,
    Subscription, Transaction, TransactionStatus, TxOutEntry,
};

/// Trait for all **async** methods.
#[async_trait]
pub trait TitanApiAsync {
    /// Returns the node's status (e.g., network info, block height).
    async fn get_status(&self) -> Result<Status, Error>;

    /// Returns the current best block tip (height + hash).
    async fn get_tip(&self) -> Result<BlockTip, Error>;

    /// Fetches a block (by height or hash) using `query::Block`.
    async fn get_block(&self, query: &query::Block) -> Result<Block, Error>;

    /// Given a block height, returns the block hash.
    async fn get_block_hash_by_height(&self, height: u64) -> Result<String, Error>;

    /// Returns a list of transaction IDs in a particular block.
    async fn get_block_txids(&self, query: &query::Block) -> Result<Vec<String>, Error>;

    /// Fetches address data (balance, transactions, etc.).
    async fn get_address(&self, address: &str) -> Result<AddressData, Error>;

    /// Returns a higher-level transaction object (including Runes info) by `txid`.
    async fn get_transaction(&self, txid: &str) -> Result<Transaction, Error>;

    /// Returns raw transaction bytes (binary).
    async fn get_transaction_raw(&self, txid: &str) -> Result<Vec<u8>, Error>;

    /// Returns raw transaction hex.
    async fn get_transaction_hex(&self, txid: &str) -> Result<String, Error>;

    /// Returns the status of a transaction by `txid`.
    async fn get_transaction_status(&self, txid: &str) -> Result<TransactionStatus, Error>;

    /// Broadcasts a transaction (raw hex) to the network and returns the resulting `Txid`.
    async fn send_transaction(&self, tx_hex: String) -> Result<Txid, Error>;

    /// Fetches a specific output by outpoint (`<txid>:<vout>`).
    async fn get_output(&self, outpoint: &str) -> Result<TxOutEntry, Error>;

    /// Returns `(HTTP Headers, Bytes)` for an inscription by its `inscription_id`.
    async fn get_inscription(&self, inscription_id: &str) -> Result<(HeaderMap, Vec<u8>), Error>;

    /// Lists existing runes, supporting pagination.
    async fn get_runes(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<PaginationResponse<RuneResponse>, Error>;

    /// Fetches data about a specific rune.
    async fn get_rune(&self, rune: &str) -> Result<RuneResponse, Error>;

    /// Returns a paginated list of `Txid` for all transactions involving a given `rune`.
    async fn get_rune_transactions(
        &self,
        rune: &str,
        pagination: Option<Pagination>,
    ) -> Result<PaginationResponse<Txid>, Error>;

    /// Returns a list of all txids currently in the mempool.
    async fn get_mempool_txids(&self) -> Result<Vec<Txid>, Error>;

    /// Fetches a single subscription by `id`.
    async fn get_subscription(&self, id: &str) -> Result<Subscription, Error>;

    /// Lists all subscriptions currently known.
    async fn list_subscriptions(&self) -> Result<Vec<Subscription>, Error>;

    /// Adds (creates) a subscription.
    async fn add_subscription(&self, subscription: &Subscription) -> Result<Subscription, Error>;

    /// Deletes a subscription by `id`.
    async fn delete_subscription(&self, id: &str) -> Result<(), Error>;
}

/// Trait for all **blocking** (synchronous) methods.
pub trait TitanApiSync {
    /// Returns the node's status in a **blocking** manner.
    fn get_status(&self) -> Result<Status, Error>;

    /// Returns the block tip in a **blocking** manner.
    fn get_tip(&self) -> Result<BlockTip, Error>;

    /// Fetches a block by height/hash in a **blocking** manner.
    fn get_block(&self, query: &query::Block) -> Result<Block, Error>;

    /// Given a block height, returns the block hash in a **blocking** manner.
    fn get_block_hash_by_height(&self, height: u64) -> Result<String, Error>;

    /// Returns txids for a block in a **blocking** manner.
    fn get_block_txids(&self, query: &query::Block) -> Result<Vec<String>, Error>;

    /// Returns address data in a **blocking** manner.
    fn get_address(&self, address: &str) -> Result<AddressData, Error>;

    /// Returns a transaction (with runic info) by `txid` in a **blocking** manner.
    fn get_transaction(&self, txid: &str) -> Result<Transaction, Error>;

    /// Returns raw tx bytes in a **blocking** manner.
    fn get_transaction_raw(&self, txid: &str) -> Result<Vec<u8>, Error>;

    /// Returns raw tx hex in a **blocking** manner.
    fn get_transaction_hex(&self, txid: &str) -> Result<String, Error>;

    /// Returns the status of a transaction by `txid` in a **blocking** manner.
    fn get_transaction_status(&self, txid: &str) -> Result<TransactionStatus, Error>;

    /// Broadcasts a raw-hex transaction in a **blocking** manner.
    fn send_transaction(&self, tx_hex: String) -> Result<Txid, Error>;

    /// Fetches a specific output (outpoint) in a **blocking** manner.
    fn get_output(&self, outpoint: &str) -> Result<TxOutEntry, Error>;

    /// Fetches an inscription (headers + bytes) by `inscription_id`, blocking.
    fn get_inscription(&self, inscription_id: &str) -> Result<(HeaderMap, Vec<u8>), Error>;

    /// Returns paginated runes in a **blocking** manner.
    fn get_runes(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<PaginationResponse<RuneResponse>, Error>;

    /// Fetches data for a specific rune in a **blocking** manner.
    fn get_rune(&self, rune: &str) -> Result<RuneResponse, Error>;

    /// Returns transactions for a given rune in a **blocking** manner.
    fn get_rune_transactions(
        &self,
        rune: &str,
        pagination: Option<Pagination>,
    ) -> Result<PaginationResponse<Txid>, Error>;

    /// Returns mempool txids in a **blocking** manner.
    fn get_mempool_txids(&self) -> Result<Vec<Txid>, Error>;

    /// Fetches a single subscription by `id`, blocking.
    fn get_subscription(&self, id: &str) -> Result<Subscription, Error>;

    /// Lists all subscriptions, blocking.
    fn list_subscriptions(&self) -> Result<Vec<Subscription>, Error>;

    /// Adds a new subscription, blocking.
    fn add_subscription(&self, subscription: &Subscription) -> Result<Subscription, Error>;

    /// Deletes a subscription by `id`, blocking.
    fn delete_subscription(&self, id: &str) -> Result<(), Error>;
}
