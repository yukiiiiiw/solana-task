use anchor_lang::prelude::*;
pub mod instructions; // 目录
use instructions::{spl::*, stack::*}; // 模块

declare_id!("9fFVjtHSekgUgzAvrPrN2NptvQLbwGsTBizDZsxsAAFr");

/** Task2
    掌握spl-token的基本用法
    任务描述
    在Task1的基础上，将存solana更改为储存spl token
*/
#[program]
pub mod task_2 {
    use super::*;

    pub fn create_token(ctx: Context<CreateSpl>, metadata: InitTokenParams) -> Result<()> {
        msg!("test log");
        crate::instructions::spl::create_token(ctx, metadata)
    }

    pub fn mint_spl(ctx: Context<MintSpl>, amount: u64) -> Result<()> {
        crate::instructions::spl::mint_spl(ctx, amount)
    }

    pub fn deposit_spl(ctx: Context<DepositSpl>, amount: u64) -> Result<()> {
        crate::instructions::stack::deposit_spl(ctx, amount)
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSpl>) -> Result<()>{
        crate::instructions::stack::withdraw_spl(ctx)
    }
}
