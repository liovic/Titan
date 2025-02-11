use {
    crate::inscription_id::InscriptionId,
    bitcoin::Txid,
    ordinals::{RuneId, SpacedRune},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct MintResponse {
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub mintable: bool,
    pub cap: u128,
    pub amount: u128,
    pub mints: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuneResponse {
    pub id: RuneId,
    pub block: u64,
    pub burned: u128,
    pub divisibility: u8,
    pub etching: Txid,
    pub number: u64,
    pub premine: u128,
    pub supply: u128,
    pub max_supply: u128,
    pub spaced_rune: SpacedRune,
    pub symbol: Option<char>,
    pub mint: Option<MintResponse>,
    pub pending_burns: u128,
    pub pending_mints: u128,
    pub inscription_id: Option<InscriptionId>,
    pub timestamp: u64,
    pub turbo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuneAmount {
    pub rune_id: RuneId,
    pub amount: String,
}
