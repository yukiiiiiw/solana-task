use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, // 处理关联代币账户的功能
    metadata::{
        create_metadata_accounts_v3,       // 创建元数据账户的功能
        mpl_token_metadata::types::DataV2, // 元数据的结构体定义
        CreateMetadataAccountsV3,          // 创建元数据账户的指令结构体
        Metadata as Metaplex,              // Metadata
    },
    token::{
        mint_to,      // 铸币功能
        Mint,         // 代币铸造的结构体
        MintTo,       // 铸币指令的结构体
        Token,        // 代币的基本功能
        TokenAccount, // 代币账户的结构体
    },
};

pub fn create_token(ctx: Context<CreateSpl>, metadata: InitTokenParams) -> Result<()> {

    // Metadata 
    let token_data: DataV2 = DataV2 {
        name: metadata.name,
        symbol: metadata.symbol,
        uri: metadata.uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    // 签名
    let bump = ctx.bumps.mint;
    let seeds = &[b"mint".as_ref(), &[bump]];
    let signer = [&seeds[..]];

    // ctx 构建Metadata账户
    let metadata_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            payer: ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.mint.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            mint_authority: ctx.accounts.mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        &signer,
    );

    // tx 创建Metadata账户
    msg!("Creating token metadata...");
    let _tx = create_metadata_accounts_v3(metadata_ctx, token_data, true, true, None)?;

    msg!("SPL Token Mint Created: {}", ctx.accounts.mint.key());
    Ok(())
}

pub fn mint_spl(ctx: Context<MintSpl>, amount: u64) -> Result<()> {
    // 签名
    let bump = ctx.bumps.mint;
    let seeds = &[b"mint".as_ref(), &[bump]];
    let signer = [&seeds[..]];

    let mint_to_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.payer_ata.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
        },
        &signer,
    );
    let _tx = mint_to(mint_to_ctx, amount)?;
    msg!("Minted {} tokens to {}", amount, ctx.accounts.payer_ata.key());
    Ok(())
}

// 元数据参数
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}

#[derive(Accounts)]
pub struct CreateSpl<'info> {
    #[account(mut, signer)]
    pub payer: Signer<'info>,  // 付费

    #[account(
        init,
        payer = payer,
        seeds = [b"mint"],
        bump,
        mint::decimals = 9,  
        mint::authority = mint, // mint权限归当前合约
        mint::freeze_authority = mint, // 冻结权限归当前合约
    )]
    pub mint: Account<'info, Mint>,  // mint账户

    /// CHECK: This is a PDA derived from the Mint account, verified in the instruction
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>, // 管理元数据账户

    pub rent: Sysvar<'info, Rent>, // 租金管理账户
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>, // SPL
    pub token_metadata_program: Program<'info, Metaplex>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct MintSpl<'info> {
    #[account(mut, signer)]
    pub payer: Signer<'info>, 

    #[account(
        mut,
        seeds = [b"mint"],
        bump,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,  // 代币的 Mint 账户

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub payer_ata: Account<'info, TokenAccount>,  // 用户的 Associated Token Account (ATA)
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>, // SPL
    pub token_metadata_program: Program<'info, Metaplex>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
