use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("AxPSkLsnEeARLmBtBnNd5fxFgLRqs1tnfzCgvK3fhvJy");

#[program]
pub mod defind {
    use super::*;
    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     let counter = &mut ctx.accounts.counter;
    //     counter.count = 0;
    //     msg!("Counter Account Created");
    //     msg!("Current Count: { }", counter.count);
    //     Ok(())
    // }

    pub fn create(ctx: Context<Create>, name: String) -> ProgramResult {
        let fund = &mut ctx.accounts.fund;

        if fund.name != "" {
            msg!("Fund already created");
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        fund.name = name;
        fund.balance = 0;
        fund.initial_deposits = 0;
        fund.owner = *ctx.accounts.user.key;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        msg!("New Deposit found");
        msg!("Amount: { }", amount);

        // fn increment(ctx: Context<Update>) -> Result<()> {
        //     let counter = &mut ctx.accounts.counter;
        //     msg!("Previous counter: {}", counter.count);
        //     counter.count = counter.count.checked_add(1).unwrap();
        //     msg!("Counter incremented. Current count: {}", counter.count);
        //     Ok(())
        // }

        if amount == 0 {
            return Err(ProgramError::InvalidArgument);
        }

        let deposit_data = &mut ctx.accounts.data;
        deposit_data.fund = ctx.accounts.fund.key();
        msg!("fund: { }", deposit_data.fund);
        if deposit_data.deposits != 0 {

            let txn = anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.user.key(),
                &ctx.accounts.fund.key(),
                amount
            );
            anchor_lang::solana_program::program::invoke(
                &txn,
                &[
                    ctx.accounts.user.to_account_info(),
                    ctx.accounts.fund.to_account_info()
                ],
            )?;
            (&mut ctx.accounts.fund).initial_deposits += amount;
            (&mut ctx.accounts.fund).balance += amount;

            deposit_data.deposits += amount;
            deposit_data.share = (deposit_data.deposits as f32 / (**ctx.accounts.fund.to_account_info().try_borrow_mut_lamports()?) as f32) as f32;

            msg!("Deposits: { }", deposit_data.deposits);
            msg!("Share: { }", deposit_data.share);
            Ok(())
        } else {
            deposit_data.owner = ctx.accounts.user.key();
            msg!("owner: {}", deposit_data.owner);

            let txn = anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.user.key(),
                &ctx.accounts.fund.key(),
                amount
            );
            anchor_lang::solana_program::program::invoke(
                &txn,
                &[
                    ctx.accounts.user.to_account_info(),
                    ctx.accounts.fund.to_account_info(),
                ],
            )?;
            (&mut ctx.accounts.fund).initial_deposits += amount;
            (&mut ctx.accounts.fund).balance += amount;

            deposit_data.deposits = amount;
            deposit_data.share = (deposit_data.deposits as f32 / (**ctx.accounts.fund.to_account_info().try_borrow_mut_lamports()?) as f32) as f32;

            msg!("Deposits: { }", deposit_data.deposits);
            msg!("Share: { }", deposit_data.share);
            Ok(())
        }
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let fund = &mut ctx.accounts.fund;
        let user = &mut ctx.accounts.user;
        let deposit_data = &mut ctx.accounts.data;

        if deposit_data.owner != user.key() {
            return Err(ProgramError::IncorrectProgramId);
        }

        if deposit_data.fund != fund.key() {
            return Err(ProgramError::IncorrectProgramId)
        }

        let rent = Rent::get()?.minimum_balance(fund.to_account_info().data_len());
        if deposit_data.deposits - rent < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        **fund.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

        deposit_data.deposits -= amount;
        deposit_data.share = deposit_data.deposits as f32 / (**ctx.accounts.fund.to_account_info().try_borrow_mut_lamports()?) as f32;

        if deposit_data.share == 0.0 {
            fn close(_ctx: Context<Close>) -> Result<()> {
                msg!("Closing Data account");
                Ok(())
            }
        }

        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(init, payer = user, space = 8 + 8)]
//     pub counter: Account<'info, Counter>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }
//
// #[derive(Accounts)]
// pub struct Update<'info> {
//     #[account(mut)]
//     pub counter: Account<'info, Counter>,
//     pub user: Signer<'info>,
// }

#[derive(Accounts)]
#[instruction(name:String)]
pub struct Create<'info> {
    #[account(
        init,
        payer = user,
        space = 32 + 1 + 32 + 1,
        seeds = [name.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub fund: Account<'info, Fund>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Fund {
    pub name: String,
    pub balance: u64,
    pub owner: Pubkey,
    pub initial_deposits: u64,
}

#[account]
pub struct DepositData {
    pub owner: Pubkey, //32
    pub deposits: u64, //1
    pub share: f32, //4
    pub fund: Pubkey, //32
}

// #[account]
// pub struct Counter {
//     pub count: u64,
// }

#[derive(Accounts)]
#[instruction()]
pub struct Deposit<'info> {
    #[account(mut)]
    pub fund: Account<'info, Fund>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        seeds = [b"dataaccount", user.key().as_ref()],
        bump,
        payer = user,
        space = 100
    )]
    pub data: Account<'info, DepositData>,
    //#[account(mut)]
    //pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction()]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub fund: Account<'info, Fund>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    mut,
    seeds = [b"dataaccount", user.key().as_ref()],
    bump,
    realloc = 32 + 1 + 4,
    realloc::payer = user,
    realloc::zero = true
    )]
    pub data: Account<'info, DepositData>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(
        mut,
        seeds = [b"dataaccount", user.key().as_ref()],
        bump,
        close = user
    )]
    pub data_account: Account<'info, DepositData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}