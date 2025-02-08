use bitcoin::Txid;
use reqwest::{header::HeaderMap, Client as AsyncReqwestClient};
use std::str::FromStr;
use titan_types::*;

use crate::Error;

use super::TitanApiAsync;

#[derive(Clone)]
pub struct AsyncClient {
    /// The async HTTP client from `reqwest`.
    http_client: AsyncReqwestClient,
    /// The base URL for all endpoints (e.g. http://localhost:3030).
    base_url: String,
}

impl AsyncClient {
    /// Creates a new `AsyncClient` for the given `base_url`.
    pub fn new(base_url: &str) -> Self {
        Self {
            http_client: AsyncReqwestClient::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }
}

#[async_trait::async_trait]
impl TitanApiAsync for AsyncClient {
    async fn get_status(&self) -> Result<Status, Error> {
        let url = format!("{}/status", self.base_url);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn get_tip(&self) -> Result<BlockTip, Error> {
        let url = format!("{}/tip", self.base_url);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn get_block(&self, query: &query::Block) -> Result<Block, Error> {
        let url = format!("{}/block/{}", self.base_url, query);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn get_block_hash_by_height(&self, height: u64) -> Result<String, Error> {
        let url = format!("{}/block/{}/hash", self.base_url, height);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.text().await?)
    }

    async fn get_block_txids(&self, query: &query::Block) -> Result<Vec<String>, Error> {
        let url = format!("{}/block/{}/txids", self.base_url, query);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn get_address(&self, address: &str) -> Result<AddressData, Error> {
        let url = format!("{}/address/{}", self.base_url, address);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn get_transaction(&self, txid: &str) -> Result<Transaction, Error> {
        let url = format!("{}/tx/{}", self.base_url, txid);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn get_transaction_raw(&self, txid: &str) -> Result<Vec<u8>, Error> {
        let url = format!("{}/tx/{}/raw", self.base_url, txid);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.bytes().await?.to_vec())
    }

    async fn get_transaction_hex(&self, txid: &str) -> Result<String, Error> {
        let url = format!("{}/tx/{}/hex", self.base_url, txid);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.text().await?)
    }

    async fn get_transaction_status(&self, txid: &str) -> Result<TransactionStatus, Error> {
        let url = format!("{}/tx/{}/status", self.base_url, txid);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn send_transaction(&self, tx_hex: String) -> Result<Txid, Error> {
        let url = format!("{}/tx/broadcast", self.base_url);
        let resp = self.http_client.post(&url).body(tx_hex).send().await?;
        let status = resp.status();
        let body_text = resp.text().await?;
        if !status.is_success() {
            return Err(Error::Runtime(format!(
                "Broadcast failed: HTTP {} - {}",
                status, body_text
            )));
        }
        let txid = Txid::from_str(&body_text)?;
        Ok(txid)
    }

    async fn get_output(&self, outpoint: &str) -> Result<TxOutEntry, Error> {
        let url = format!("{}/output/{}", self.base_url, outpoint);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn get_inscription(&self, inscription_id: &str) -> Result<(HeaderMap, Vec<u8>), Error> {
        let url = format!("{}/inscription/{}", self.base_url, inscription_id);
        let resp = self.http_client.get(&url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Runtime(format!(
                "Inscription request failed: HTTP {} - {}",
                status, body
            )));
        }
        let headers = resp.headers().clone();
        let bytes = resp.bytes().await?.to_vec();
        Ok((headers, bytes))
    }

    async fn get_runes(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<PaginationResponse<RuneResponse>, Error> {
        let url = format!("{}/runes", self.base_url);
        let mut req = self.http_client.get(&url);
        if let Some(ref p) = pagination {
            req = req.query(&[("skip", p.skip), ("limit", p.limit)]);
        }
        let resp = req.send().await?;
        Ok(resp.json().await?)
    }

    async fn get_rune(&self, rune: &str) -> Result<RuneResponse, Error> {
        let url = format!("{}/rune/{}", self.base_url, rune);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn get_rune_transactions(
        &self,
        rune: &str,
        pagination: Option<Pagination>,
    ) -> Result<PaginationResponse<Txid>, Error> {
        let url = format!("{}/rune/{}/transactions", self.base_url, rune);
        let mut req = self.http_client.get(&url);
        if let Some(ref p) = pagination {
            req = req.query(&[("skip", p.skip), ("limit", p.limit)]);
        }
        let resp = req.send().await?;
        Ok(resp.json().await?)
    }

    async fn get_mempool_txids(&self) -> Result<Vec<Txid>, Error> {
        let url = format!("{}/mempool/txids", self.base_url);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn get_subscription(&self, id: &str) -> Result<Subscription, Error> {
        let url = format!("{}/subscription/{}", self.base_url, id);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn list_subscriptions(&self) -> Result<Vec<Subscription>, Error> {
        let url = format!("{}/subscriptions", self.base_url);
        let resp = self.http_client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    async fn add_subscription(&self, subscription: &Subscription) -> Result<Subscription, Error> {
        let url = format!("{}/subscription", self.base_url);
        let resp = self
            .http_client
            .post(&url)
            .json(subscription)
            .send()
            .await?;
        Ok(resp.json().await?)
    }

    async fn delete_subscription(&self, id: &str) -> Result<(), Error> {
        let url = format!("{}/subscription/{}", self.base_url, id);
        let resp = self.http_client.delete(&url).send().await?;
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(Error::Runtime(format!(
                "Delete subscription failed: HTTP {} - {}",
                status, body
            )));
        }
        Ok(())
    }
}
