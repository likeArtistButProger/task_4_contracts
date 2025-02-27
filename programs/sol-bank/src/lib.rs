use anchor_lang::prelude::*;

declare_id!("GpKvZLjywQaz1PSZycVwWuRhFKZk1n3Cw2kGUns8k2Y3");

#[program]
pub mod sol_bank {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.owner = *ctx.accounts.user.key;
        deposit_account.balance = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let user = &ctx.accounts.owner;

        **deposit_account.to_account_info().try_borrow_mut_lamports()? += amount;
        **user.to_account_info().try_borrow_mut_lamports()? -= amount;

        deposit_account.balance += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let user = &ctx.accounts.owner;

        require!(deposit_account.balance >= amount, SolBankError::InsufficientFunds);

        **deposit_account.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

        deposit_account.balance -= amount;
        Ok(())
    }

    pub fn get_balance(ctx: Context<GetBalance>) -> Result<u64> {
        let deposit_account = &ctx.accounts.deposit_account;
        Ok(deposit_account.balance)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"deposit", user.key().as_ref()],
        bump
    )]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"deposit", owner.key().as_ref()],
        bump
    )]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"deposit", deposit_account.owner.as_ref()],
        bump,
        has_one = owner
    )]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(mut, address = deposit_account.owner)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetBalance<'info> {
    #[account(
        seeds = [b"deposit", deposit_account.owner.as_ref()],
        bump,
        has_one = owner
    )]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(address = deposit_account.owner)]
    pub owner: Signer<'info>,
}

#[account]
pub struct DepositAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

#[error_code]
pub enum SolBankError {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
