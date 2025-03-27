use chia::{
    clvm_traits::{self, FromClvm, ToClvm},
    clvm_utils::TreeHash,
};
use chia_wallet_sdk::types::Mod;
use hex_literal::hex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToClvm, FromClvm)]
#[clvm(curry)]
pub struct LinearBidProgramArgs {
    pub increase_amount: u64,
}

impl LinearBidProgramArgs {
    pub fn new(increase_amount: u64) -> Self {
        Self { increase_amount }
    }
}

impl Mod for LinearBidProgramArgs {
    const MOD_REVEAL: &[u8] = &hex!(
        "
        ff02ffff03ffff15ffff12ff02ff0980ffff11ff0bff0d8080ffff01ff0880ff
        ff010d80ff0180
        "
    );

    const MOD_HASH: TreeHash = TreeHash::new(hex!(
        "3a42383532b60c6f1d4cf4b88146ec7ebbd1a7caf39f85cbf9bc66b0361898c2"
    ));
}
