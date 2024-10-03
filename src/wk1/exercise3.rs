/*
Exercise 3 - Security analysis
Analyze the following piece of code with a security mindset and answer to the following:
Explain what the function does.
What could go wrong?
How to fix it?
Note: This code is taken from a Solana program written with Anchor, but it has been heavily simplified.
Note: DECIMALS_SCALAR value is not given. It is an u128 which may have any value except 0.
Note: exchange_rate.deposit_rate is an u64.
*/

// pub fn deposit(ctx: Context<Deposit>, collat: u64) -> Result<()> {
//     let rate = exchange_rate.deposit_rate as u128;
//     let amt = (collat as u128 * rate / DECIMALS_SCALAR) as u64;

//     // transfer(token, from, to, amount)
//     token::transfer(collateral_token, ctx.caller, ctx.this, collat)?;

//     // mint_to(token, to, amount)
//     token::mint_to(shares_token, ctx.caller, amt)?;

//     Ok(())
// }

/*
    Explain what the function does -
    public deposit function  calcs shares on collateral, xfers collateral to contract, mints shares to user

    What could go wrong?
    int overflow, precision loss or divide by zero, mint 0 shares, re-entrancy?, no logging, no validation or error handling
*/

// How to fix it?
pub fn deposit(ctx: Context<Deposit>, collat: u64) -> Result<()> {
    let rate = u128::from(exchange_rate.deposit_rate);
    let amt = u64::try_from(collat as u128 * rate / DECIMALS_SCALAR)
        .map_err(|_| ErrorCode::AmountOverflow)?;

    if amt == 0 {
        return Err(ErrorCode::ZeroAmount.into());
    }

    // transfer(token, from, to, amount)
    token::transfer(collateral_token, ctx.caller, ctx.this, collat)?;

    // mint_to(token, to, amount)
    token::mint_to(shares_token, ctx.caller, amt)?;

    Ok(())
}
