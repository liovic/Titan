use {
    borsh::{BorshDeserialize, BorshSerialize},
    ordinals::RuneId,
    serde::{Deserialize, Serialize},
    std::io::{Read, Result, Write},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuneAmount {
    pub rune_id: RuneId,
    pub amount: u128,
}

impl RuneAmount {
    pub fn to_cents(&self) -> titan_types::RuneAmount {
        titan_types::RuneAmount {
            rune_id: self.rune_id,
            amount: self.amount.to_string(),
        }
    }

    pub fn to_decimals(&self, divibility: u8) -> titan_types::RuneAmount {
        let cutoff = 10u128.checked_pow(divibility.into()).unwrap();

        let whole = self.amount / cutoff;
        let mut fractional = self.amount % cutoff;

        let amount = if fractional == 0 {
            format!("{whole}")
        } else {
            let mut width = usize::from(divibility);
            while fractional % 10 == 0 {
                fractional /= 10;
                width -= 1;
            }

            format!("{whole}.{fractional:0>width$}")
        };

        titan_types::RuneAmount {
            rune_id: self.rune_id,
            amount,
        }
    }
}

impl From<(RuneId, u128)> for RuneAmount {
    fn from((rune_id, amount): (RuneId, u128)) -> Self {
        Self { rune_id, amount }
    }
}

impl BorshSerialize for RuneAmount {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        // Write out RuneId (block, tx):
        BorshSerialize::serialize(&self.rune_id.block, writer)?;
        BorshSerialize::serialize(&self.rune_id.tx, writer)?;

        // Write out amount
        BorshSerialize::serialize(&self.amount, writer)?;

        Ok(())
    }
}

impl BorshDeserialize for RuneAmount {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        // Read back RuneId fields:
        let block = u64::deserialize_reader(reader)?;
        let tx = u32::deserialize_reader(reader)?;

        // Read back amount
        let amount = u128::deserialize_reader(reader)?;

        Ok(RuneAmount {
            rune_id: RuneId { block, tx },
            amount,
        })
    }
}
