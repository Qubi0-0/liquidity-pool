use std::fmt::Error;

use std::fmt;

#[derive(Debug)]
pub enum LpPoolError {
    InvalidFee,
    InsufficientLiquidity,
    InsufficientStakedTokens,
    InvalidTokenAmount,
}

impl fmt::Display for LpPoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LpPoolError::InvalidFee => write!(f, "Invalid fee values provided."),
            LpPoolError::InsufficientLiquidity => write!(f, "Insufficient liquidity in the pool."),
            LpPoolError::InsufficientStakedTokens => write!(f, "Insufficient staked tokens in the pool."),
            LpPoolError::InvalidTokenAmount => write!(f, "Invalid token amount provided."),
        }
    }
}

impl std::error::Error for LpPoolError {}


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

/// Represents the precision factor used for decimal shifting.
const PRECISION_FACTOR: u64 = 0x1_0000_0000u64;

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
    /// * `liquidity_target` - The target amount of liquidity for the pool.
    /// * `min_fee` - The minimum fee percentage.
    /// * `max_fee` - The maximum fee percentage.
    ///
    ///
    /// Calculates :
    /// * `token_amount` - The amount of tokens in the pool.
    /// * `st_token_amount` - The amount of staked tokens in the pool.
    /// * `lp_token_amount` - The amount of LP tokens in the pool.
    /// # Returns
    ///
    /// A result containing the initialized `LpPool` or an error.
    pub fn init(
        price: f64,
        liquidity_target: f64,
        min_fee: f64,
        max_fee: f64,
    ) -> Result<Self, LpPoolError> {
        if max_fee > 100.0 || min_fee < 0.0 || (min_fee > max_fee) || liquidity_target <= 0.0 {
            return Err(LpPoolError::InvalidFee);
        }
        // decimal shifting to provide float-like precision
        let price = Price((price * PRECISION_FACTOR as f64).round() as u64);
        let liquidity_target =
            TokenAmount((liquidity_target * PRECISION_FACTOR as f64).round() as u64);
        let min_fee = Percentage((min_fee * PRECISION_FACTOR as f64).round() as u64);
        let max_fee = Percentage((max_fee * PRECISION_FACTOR as f64).round() as u64);

        let token_amount = TokenAmount(liquidity_target.0 / 2); // Example logic
        let st_token_amount = StakedTokenAmount(liquidity_target.0 / 4); // Example logic
        let lp_token_amount = LpTokenAmount(liquidity_target.0 / 2); // Example logic
        
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
    pub fn add_liquidity(&mut self, token_amount: f64) -> Result<f64, LpPoolError> {
        let new_tokens_u64 = (token_amount * PRECISION_FACTOR as f64).round() as u64;
    
        if token_amount <= 0.0 {
            return Err(LpPoolError::InvalidTokenAmount);
        }
    
        // Split the added liquidity between token_amount and st_token_amount
        let tokens_to_add = new_tokens_u64 / 2; // 50% regular tokens
        let staked_tokens_to_add = new_tokens_u64 - tokens_to_add; // Remaining 50% to staked tokens
    
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
    pub fn remove_liquidity(&mut self, lp_token_amount: f64) -> Result<(f64, f64), LpPoolError> {
        let lp_token_amount_u64 = (lp_token_amount * PRECISION_FACTOR as f64).round() as u64;
        let unstake_fee = self.max_fee.0
            - (self.max_fee.0 - self.min_fee.0) * lp_token_amount_u64 / self.liquidity_target.0;
        if self.lp_token_amount.0 < lp_token_amount.0 {
            return Err(LpPoolError::InsufficientLiquidity);
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
    pub fn swap(&mut self, staked_token_amount: f64) -> Result<f64, LpPoolError> {
        if staked_token_amount <= 0.0 {
            return Err(LpPoolError::InvalidTokenAmount);
        }
    
        let staked_token_u64 =
            StakedTokenAmount((staked_token_amount * PRECISION_FACTOR as f64).round() as u64);
    
        if staked_token_u64.0 > self.st_token_amount.0 {
            return Err(LpPoolError::InsufficientStakedTokens);
        }
    
        let swap_ratio = self.price.0 as f64 / PRECISION_FACTOR as f64;
        let tokens_received_u64 = ((staked_token_u64.0 as f64 * swap_ratio) * PRECISION_FACTOR as f64).round() as u64 / PRECISION_FACTOR;
    
        self.st_token_amount.0 -= staked_token_u64.0;
        self.token_amount.0 += tokens_received_u64;
    
        Ok(tokens_received_u64 as f64 / PRECISION_FACTOR as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
