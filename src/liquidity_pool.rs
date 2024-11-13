use std::fmt::Error;

struct TokenAmount(u64);
struct StakedTokenAmount(u64);
struct LpTokenAmount(u64);
struct Price(u64);
struct Percentage(u64);

struct LpPool {
    price: Price,
    token_amount: TokenAmount,
    st_token_amount: StakedTokenAmount,
    lp_token_amount: LpTokenAmount,
    liquidity_target: TokenAmount,
    min_fee: Percentage,
    max_fee: Percentage,
}

impl LpPool {
    pub fn init(
        price: Price,
        token_amount: TokenAmount,
        st_token_amount: StakedTokenAmount,
        lp_token_amount: LpTokenAmount,
        liquidity_target: TokenAmount,
        min_fee: Percentage,
        max_fee: Percentage,
    ) -> Result<Self, Error> {
        todo!()
    }
    pub fn add_liquidity(
        self: &mut Self,
        token_amount: TokenAmount,
    ) -> Result<LpTokenAmount, Error> {
        todo!()
    }
    pub fn remove_liquidity(
        self: &mut Self,
        lp_token_amount: LpTokenAmount,
    ) -> Result<(TokenAmount, StakedTokenAmount), Error> {
        todo!()
    }

    pub fn swamp(
        self: &mut Self,
        staked_token_amount: StakedTokenAmount,
    ) -> Result<TokenAmount, Error> {
        todo!()
    }
}
