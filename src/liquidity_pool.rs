use std::fmt::Error;

/// Represents an amount of tokens.
pub struct TokenAmount(pub u64);

/// Represents an amount of staked tokens.
pub struct StakedTokenAmount(pub u64);

/// Represents an amount of LP tokens.
pub struct LpTokenAmount(pub u64);

/// Represents the price of a token.
pub struct Price(pub u64);

/// Represents a percentage value.
pub struct Percentage(pub u64);

/// Represents a liquidity pool with various parameters.
pub struct LpPool {
    pub price: Price,
    pub token_amount: TokenAmount,
    pub st_token_amount: StakedTokenAmount,
    pub lp_token_amount: LpTokenAmount,
    pub liquidity_target: TokenAmount,
    pub min_fee: Percentage,
    pub max_fee: Percentage,
}

impl LpPool {
    /// Initializes a new liquidity pool with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `price` - The price of the token.
    /// * `token_amount` - The amount of tokens in the pool.
    /// * `st_token_amount` - The amount of staked tokens in the pool.
    /// * `lp_token_amount` - The amount of LP tokens in the pool.
    /// * `liquidity_target` - The target amount of liquidity for the pool.
    /// * `min_fee` - The minimum fee percentage.
    /// * `max_fee` - The maximum fee percentage.
    ///
    /// # Returns
    ///
    /// A result containing the initialized `LpPool` or an error.
    pub fn init(
        price: Price,
        token_amount: TokenAmount,
        st_token_amount: StakedTokenAmount,
        lp_token_amount: LpTokenAmount,
        liquidity_target: TokenAmount,
        min_fee: Percentage,
        max_fee: Percentage,
    ) -> Result<Self, Error> {
        Ok(LpPool {
            price,
            token_amount,
            st_token_amount,
            lp_token_amount,
            liquidity_target,
            min_fee,
            max_fee,
        })
    }

    /// Adds liquidity to the pool.
    ///
    /// # Arguments
    ///
    /// * `token_amount` - The amount of tokens to add to the pool.
    ///
    /// # Returns
    ///
    /// A result containing the amount of LP tokens received or an error.
    pub fn add_liquidity(&mut self, token_amount: TokenAmount) -> Result<LpTokenAmount, Error> {
        self.token_amount.0 += token_amount.0;
        let lp_token_recevied = LpTokenAmount(token_amount.0);
        self.lp_token_amount.0 += lp_token_recevied.0;
        Ok(lp_token_recevied)
    }

    /// Removes liquidity from the pool.
    ///
    /// # Arguments
    ///
    /// * `lp_token_amount` - The amount of LP tokens to remove from the pool.
    ///
    /// # Returns
    ///
    /// A result containing a tuple with the amount of tokens and staked tokens received or an error.
    pub fn remove_liquidity(
        &mut self,
        lp_token_amount: LpTokenAmount,
    ) -> Result<(TokenAmount, StakedTokenAmount), Error> {
        if self.lp_token_amount.0 < lp_token_amount.0 {
            return Err(Error);
        }
        self.lp_token_amount.0 -= lp_token_amount.0;
        let tokens_received = TokenAmount(lp_token_amount.0); // Simplified logic
        let staked_tokens_received = StakedTokenAmount(0); // Simplified logic
        self.token_amount.0 -= tokens_received.0;
        Ok((tokens_received, staked_tokens_received))
    }

    /// Swaps staked tokens for regular tokens.
    ///
    /// # Arguments
    ///
    /// * `staked_token_amount` - The amount of staked tokens to swap.
    ///
    /// # Returns
    ///
    /// A result containing the amount of tokens received or an error.
    pub fn swap(&mut self, staked_token_amount: StakedTokenAmount) -> Result<TokenAmount, Error> {
        if staked_token_amount.0 > self.st_token_amount.0 {
            return Err(Error);
        }
        self.st_token_amount.0 -= staked_token_amount.0;
        let tokens_received = TokenAmount(staked_token_amount.0); // Simplified logic
        self.token_amount.0 += tokens_received.0;
        Ok(tokens_received)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup_pool() -> LpPool {
        LpPool::init(
            Price(150),              // 1.5
            TokenAmount(10000),      // 100.0
            StakedTokenAmount(5000), // 50.0
            LpTokenAmount(10000),    // 100.0
            TokenAmount(9000),       // 90.0
            Percentage(1),           // 0.1%
            Percentage(900),         // 9%
        )
        .unwrap()
    }

    #[test]
    fn test_init() {
        let pool = setup_pool();

        assert_eq!(pool.price.0, 150);
        assert_eq!(pool.token_amount.0, 10000);
        assert_eq!(pool.st_token_amount.0, 5000);
        assert_eq!(pool.lp_token_amount.0, 10000);
        assert_eq!(pool.liquidity_target.0, 9000);
        assert_eq!(pool.min_fee.0, 1);
        assert_eq!(pool.max_fee.0, 900);
    }

    #[test]
    fn test_add_liquidity() {
        let mut pool = setup_pool();

        let result = pool.add_liquidity(TokenAmount(10000)).unwrap(); // 100.0
        assert_eq!(result.0, 10000);
        assert_eq!(pool.token_amount.0, 20000);
        assert_eq!(pool.lp_token_amount.0, 20000);
    }

    #[test]
    fn test_remove_liquidity() {
        let mut pool = setup_pool();

        let result = pool.remove_liquidity(LpTokenAmount(5000)).unwrap(); // 50.0
        assert_eq!(result.0 .0, 5000);
        assert_eq!(result.1 .0, 0);
        assert_eq!(pool.token_amount.0, 5000);
        assert_eq!(pool.lp_token_amount.0, 5000);
    }

    #[test]
    fn test_swap() {
        let mut pool = setup_pool();

        let result = pool.swap(StakedTokenAmount(2000)).unwrap(); // 20.0
        assert_eq!(result.0, 2000);
        assert_eq!(pool.st_token_amount.0, 3000);
        assert_eq!(pool.token_amount.0, 12000);
    }
}
