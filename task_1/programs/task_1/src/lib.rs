use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("62GKaiorngxb3x15sqHL8SPZEiz2EyxkiRUJVUQcZ9Zf");

/**
* Task1
* 功能概述
    1. deposit
    质押：用户支付solana，并将用户支付的数量记录下来，确保可以获取用户deposit的总余额
    2. withdraw
    提款：一次性提取用户支付的所有solana
*/
#[program]
pub mod task_1 {
    use super::*;

    // 用户质押 SOL（充值）
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let user: AccountInfo<'_> = ctx.accounts.user.to_account_info();
        let stack_account = ctx.accounts.stack_account_pda.to_account_info();
        let system = ctx.accounts.system_program.to_account_info();
        let stack_account_pda_pump = ctx.accounts.stack_account.stack_account_pda_pump;
        let stack_account_pump = ctx.accounts.stack_account.stack_account_pump;

        // 用户充值到质押账户
        anchor_lang::system_program::transfer(
            CpiContext::new(
                system,
                anchor_lang::system_program::Transfer {
                    from: user,
                    to: stack_account,
                },
            ),
            amount,
        )?;

        // 更新余额
        ctx.accounts.stack_account.balance += amount;
        // 记录bump
        if (stack_account_pda_pump == 0) {
            ctx.accounts.stack_account.stack_account_pda_pump = ctx.bumps.stack_account_pda;
        }

        if (stack_account_pump == 0) {
            ctx.accounts.stack_account.stack_account_pump = ctx.bumps.stack_account;
        }

        msg!("Deposit {} lamports to user stack account", amount);
        Ok(())
    }

    // 用户提款（提取全部 SOL）
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let user = ctx.accounts.user.to_account_info();
        let stack_account: AccountInfo<'_> = ctx.accounts.stack_account_pda.to_account_info();
        let system = ctx.accounts.system_program.to_account_info();

        // 查询
        let balance = ctx.accounts.stack_account.balance;
        let pda_balance = ctx.accounts.stack_account_pda.get_lamports();

        // 判断质押账户余额
        require!(balance > 0, CustomError::InsufficientBalance);

        // 将余额返回给用户
        // let bump = Pubkey::find_program_address(&[b"stack", user.key.as_ref()], &id()).1;
        let bump = ctx.accounts.stack_account.stack_account_pda_pump;

        msg!("find_program_address program_id {} bump {} balance:{} pda_balance:{}", &id(), bump, balance, pda_balance);
        // drop(pda_balance); // 释放borrow 仅为log

        let seeds: &[&[u8]] = &[b"stack", user.key.as_ref(), &[bump]];
        let signer = &[&seeds[..]];
        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                system,
                anchor_lang::system_program::Transfer {
                    from: stack_account,
                    to: user,
                },
                signer,
            ),
            balance,
        )?;
        ctx.accounts.stack_account.balance -= balance;

        msg!("Withdrew {} lamports to user wallet", balance);
        Ok(())
    }
}

/** 充值结构体 */
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, signer)]
    pub user: Signer<'info>, // 充值用户
    #[account(
        init_if_needed,
        payer = user,
        seeds = [b"stack", user.key.as_ref()],
        owner = system_program::ID, // 系统账户
        bump,
        space = 0 
    )]
    /// CHECK: This is a PDA verified in constraints.
    pub stack_account_pda: AccountInfo<'info>, // 存储SOL的普通质押账户[系统账户、用户钱包可以操作转账，需要签名]
    #[account(
        init_if_needed, // 如果 stack_account 未初始化，自动初始化
        payer = user, 
        seeds = [user.key.as_ref()],
        bump, 
        space = 8 + 16 // #[drive(init space)] or std::mem::size_of::<StackAccount>()
    )]
    pub stack_account: Account<'info, StackAccount>,  // 数据账户，可以随意更改不用签名
    pub system_program: Program<'info, System>,
}

/** 提现结构体 */
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, signer)]
    pub user: Signer<'info>, // 提现用户
    #[account(mut)]
    /// CHECK: This is a PDA verified in constraints.
    pub stack_account_pda: AccountInfo<'info>, // 存储SOL的普通质押账户
    #[account(mut)]
    pub stack_account: Account<'info, StackAccount>, // 质押数据账户
    pub system_program: Program<'info, System>,
}

/** 质押账户结构体 */
#[account]
pub struct StackAccount {
    pub balance: u64, // 记录用户的质押金额
    pub stack_account_pump: u8, 
    pub stack_account_pda_pump: u8
}

/** 自定义异常 */
#[error_code]
pub enum CustomError {
    #[msg("Insufficient balance for withdrawal.")]
    InsufficientBalance,
}
