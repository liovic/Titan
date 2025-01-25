use {
    crate::index::{Chain, RpcClientError, RpcClientProvider},
    bitcoincore_rpc::{Auth, Client},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ServerConfig {
    pub(crate) chain: Chain,
    pub(crate) csp_origin: Option<String>,
    pub(crate) decompress: bool,

    pub(crate) http_listen: String,

    pub(crate) bitcoin_rpc_url: String,
    pub(crate) bitcoin_rpc_auth: Auth,

    pub(crate) enable_subscriptions: bool,
}

impl RpcClientProvider for ServerConfig {
    fn get_new_rpc_client(&self) -> Result<Client, RpcClientError> {
        Client::new(&self.bitcoin_rpc_url, self.bitcoin_rpc_auth.clone())
            .map_err(|e| RpcClientError::FailedToConnect(e.to_string()))
    }
}
