use alloy_sol_types::SolValue;
use anyhow::{anyhow, Result};
use zeth_primitives::{block::Header, ethers::from_ethers_h256, taiko::ProtocolInstance};

use crate::taiko::host::TaikoExtra;

pub fn verify(header: &Header, pi: &ProtocolInstance, extra: &TaikoExtra) -> Result<()> {
    // check the block metadata
    if pi.block_metadata.abi_encode() != extra.block_proposed.meta.abi_encode() {
        return Err(anyhow!(
            "block metadata mismatch, expected: {:?}, got: {:?}",
            extra.block_proposed.meta,
            pi.block_metadata
        ));
    }
    // check the block hash
    if Some(header.hash()) != extra.l2_fini_block.hash.map(from_ethers_h256) {
        return Err(anyhow!(
            "block hash mismatch, expected: {:?}, got: {:?}",
            extra.l2_fini_block,
            header,
        ));
    }

    println!("Protocol instance Transition: {:?}", pi.transition);
    println!("Protocol instance Metahash: {}", pi.meta_hash());
    Ok(())
}
