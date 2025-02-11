use {
    super::RuneAmount,
    borsh::{BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TxOutEntry {
    pub runes: Vec<RuneAmount>,
    pub value: u64,
    pub spent: bool,
}

impl TxOutEntry {
    pub fn has_runes(&self) -> bool {
        !self.runes.is_empty()
    }
}

impl Into<titan_types::TxOutResponse> for TxOutEntry {
    fn into(self) -> titan_types::TxOutResponse {
        titan_types::TxOutResponse {
            value: self.value,
            runes: self.runes.into_iter().map(|r| r.into()).collect(),
            spent: self.spent,
        }
    }
}
