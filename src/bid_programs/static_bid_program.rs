use chia::{
    clvm_traits::{self, FromClvm, ToClvm},
    clvm_utils::TreeHash,
};
use chia_wallet_sdk::types::Mod;
use hex_literal::hex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToClvm, FromClvm)]
#[clvm(curry)]
pub struct StaticBidProgramArgs {
    pub increase_amount: u64,
}

impl StaticBidProgramArgs {
    pub fn new(increase_amount: u64) -> Self {
        Self { increase_amount }
    }
}

impl Mod for StaticBidProgramArgs {
    const MOD_REVEAL: &[u8] = &hex!(
        "
        ff02ffff03ffff15ff02ffff11ff0bff0d8080ffff01ff0880ffff010d80ff01
        80
        "
    );

    const MOD_HASH: TreeHash = TreeHash::new(hex!(
        "e849f0912effcb4df4bc1b2fa66dfe2ef9c563c02a3e38c4f8350f9e5292dda1"
    ));
}
