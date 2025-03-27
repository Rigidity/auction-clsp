mod initial_bid_program;
mod linear_bid_program;
mod static_bid_program;

pub use initial_bid_program::*;
pub use linear_bid_program::*;
pub use static_bid_program::*;

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chia::clvm_traits::{self, FromClvm, ToClvm};
    use chia_wallet_sdk::{driver::SpendContext, types::Mod};
    use clvmr::Allocator;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, ToClvm, FromClvm)]
    #[clvm(solution)]
    pub struct BidProgramSolution {
        pub prev: PrevBid,
        pub amount: u64,
    }

    impl BidProgramSolution {
        pub fn new(prev: PrevBid, amount: u64) -> Self {
            Self { prev, amount }
        }
    }

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ToClvm, FromClvm)]
    #[clvm(list)]
    pub struct PrevBid {
        pub index: usize,
        #[clvm(rest)]
        pub amount: u64,
    }

    impl PrevBid {
        pub fn new(index: usize, amount: u64) -> Self {
            Self { index, amount }
        }
    }

    pub fn check_bid_program<T: Mod + ToClvm<Allocator>>(
        ctx: &mut SpendContext,
        args: T,
        solution: BidProgramSolution,
        expected_result: Option<u64>,
    ) -> Result<()> {
        let program = ctx.curry(args)?;
        let solution = ctx.alloc(&solution)?;

        let result = ctx
            .run(program, solution)
            .ok()
            .map(|output| ctx.extract::<u64>(output))
            .transpose()?;

        assert_eq!(result, expected_result);

        Ok(())
    }
}
