# From Task

○ Token: base unit.
○ StakedToken: unit backed by specific amounts of the Token. The exchange ratio
is determined by the Price.
○ LpToken: unit represents a share of the liquidity pool. You can mint LpTokens by
invoking the “add_liquidity” function, which requires a certain amount of Token.
Conversely, you can redeem LpTokens using the “remove_liquidity” function,
resulting in proportional amounts of Token and StakedToken being returned.

# From WEB

After reading Marinade state, you'll have marinade_state.msol_price: u64, and that is mSOL price in SOL multiplied by 0x1_0000_0000 (shifted), so to obtain mSOL/SOL as f64 you should do: let msol_price_f64: f64 = marinade_state.msol_price as f64 / 0x1_0000_0000 as f64, and then you get the true mSOL/SOL price.

unstake_fee = max_fee - (max_fee - min_fee) \* amount_after / liquidity_target

Price of mSOL = total_staked / tokens_minted <==> Staked_token = Token_amount / Lp_token_amount
