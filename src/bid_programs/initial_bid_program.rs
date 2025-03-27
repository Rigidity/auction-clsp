use chia::{
    clvm_traits::{self, FromClvm, ToClvm},
    clvm_utils::TreeHash,
};
use chia_wallet_sdk::types::Mod;
use hex_literal::hex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToClvm, FromClvm)]
#[clvm(curry)]
pub struct InitialBidProgramArgs<T> {
    pub initial_amount: u64,
    pub inner_bid_program: T,
}

impl<T> InitialBidProgramArgs<T> {
    pub fn new(initial_amount: u64, inner_bid_program: T) -> Self {
        Self {
            initial_amount,
            inner_bid_program,
        }
    }
}

impl<T> Mod for InitialBidProgramArgs<T> {
    const MOD_REVEAL: &[u8] = &hex!(
        "
        ff02ffff03ffff09ff13ff8080ffff01ff02ffff03ffff15ff02ff1780ffff01
        ff0880ff8080ff0180ffff01ff02ff05ff0bff178080ff0180
        "
    );

    const MOD_HASH: TreeHash = TreeHash::new(hex!(
        "25f1b8f632a8a165d36b394b60410628607b5c52136262c3b991cd3ce6bd920e"
    ));
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chia_wallet_sdk::driver::SpendContext;
    use rstest::rstest;

    use crate::bid_programs::tests::{check_bid_program, BidProgramSolution, PrevBid};

    use super::*;

    #[rstest]
    fn test_initial_bid(
        #[values(0, 1000)] initial_amount: u64,
        #[values(0, 500, 5000)] actual_amount: u64,
        #[values(true, false)] is_first: bool,
    ) -> Result<()> {
        let mut ctx = SpendContext::new();

        let expected_result = if is_first && actual_amount >= initial_amount {
            Some(0)
        } else {
            None
        };

        check_bid_program(
            &mut ctx,
            InitialBidProgramArgs::new(initial_amount, ()),
            BidProgramSolution::new(PrevBid::new(if is_first { 0 } else { 1 }, 0), actual_amount),
            expected_result,
        )?;

        Ok(())
    }
}
