# Liquidity Pool

### This Rust module defines a LpPool struct representing a liquidity pool with various parameters and methods to manage the pool. The key components are:

## Enums and Structs

`LpPoolError`: Enum representing possible errors in the liquidity pool operations.

`TokenAmount`, `StakedTokenAmount`, `LpTokenAmount`, `Price`, `Percentage`: Structs representing different quantities and values in the pool.

## Constants

`PRECISION_FACTOR` A constant used for decimal shifting to provide float-like precision.

## Struct LpPool

Represents a liquidity pool with parameters like price, token amounts, liquidity target, and fee percentages.

### Methods

`init`: Initializes a new liquidity pool with given parameters.

`add_liquidity`: Adds liquidity to the pool and returns the amount of LP tokens received.

`remove_liquidity`: Removes liquidity from the pool and returns the amount of tokens and staked tokens received.

`swap`: Swaps staked tokens for regular tokens and returns the amount of tokens received.

## Unit Tests

The module includes unit tests to verify the functionality of the LpPool methods.
This module provides a basic implementation of a liquidity pool with functionalities to add, remove, and swap liquidity, along with error handling and precision management.

# Progress

Despite the efforts understanding of how liquidity pools actually work it is still vague and for that reason hard to imlpement robustly. For now biggest problem is to comprehend how the amout of tokens to staked tokens should be adjusted.

The liquidity pool does not involve two actors: the `Swapper` and the `Liquidity Provider` due to ongoing problems with liquidity pool itself.
