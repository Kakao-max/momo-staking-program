use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("HJsEfnpgjEhEPa3SYcg6pchqhh2pFGSi331hTyqs5iis");

fn get_reward(
    staked_amount: u64,
    period: u64,
    lock_period: u64,
    reward_rate: u64,
    pending_amount: u64,
) -> u64 {
    return ((staked_amount as u128) * (period as u128) / (lock_period as u128)
        * (reward_rate as u128)
        / 100) as u64
        + pending_amount;
}

#[program]
mod pein_staking {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        lock_period: [u64; 4],
        reward_rate: [u64; 4],
    ) -> Result<()> {
        let staking_info = &mut ctx.accounts.staking_info;
        let staking_info_bump = ctx.bumps.staking_info;
        let staking_token_vaults_bump = ctx.bumps.staking_token_vaults;
        let reward_token_vaults_bump = ctx.bumps.reward_token_vaults;

        staking_info.lock_period = lock_period;
        staking_info.reward_rate = reward_rate;
        staking_info.staking_token_mint = ctx.accounts.staking_token_mint.key();
        staking_info.reward_token_mint = ctx.accounts.reward_token_mint.key();
        staking_info.owner = ctx.accounts.signer.key();
        staking_info.total_staked = 0;

        staking_info.bump = staking_info_bump;
        staking_info.staking_vaults_bump = staking_token_vaults_bump;
        staking_info.reward_vaults_bump = reward_token_vaults_bump;

        Ok(())
    }



}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, seeds = [b"staking_info"], bump, space = 10000)]
    pub staking_info: Account<'info, StakingInfo>,
    #[account(mut)]
    pub staking_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub reward_token_mint: Account<'info, Mint>,
    #[account(init, payer = signer, seeds = [b"staking_token_vaults", staking_token_mint.key().as_ref()], bump, token::mint = staking_token_mint, token::authority = staking_token_vaults)]
    pub staking_token_vaults: Account<'info, TokenAccount>,
    #[account(init, payer = signer, seeds = [b"reward_token_vaults", reward_token_mint.key().as_ref()], bump, token::mint = reward_token_mint, token::authority = reward_token_vaults)]
    pub reward_token_vaults: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawRewardtoken<'info> {
    #[account(mut, seeds = [b"staking_info"], bump = staking_info.bump)]
    pub staking_info: Account<'info, StakingInfo>,
    #[account(mut)]
    pub reward_token_mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"reward_token_vaults", reward_token_mint.key().as_ref()], bump = staking_info.reward_vaults_bump)]
    pub reward_token_vaults: Account<'info, TokenAccount>,

    #[account(mut, token::mint = staking_info.reward_token_mint)]
    pub recipient_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DepositRewardToken<'info> {
    #[account(mut, seeds = [b"staking_info"], bump = staking_info.bump)]
    pub staking_info: Account<'info, StakingInfo>,
    #[account(mut)]
    pub reward_token_mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"reward_token_vaults", reward_token_mint.key().as_ref()], bump = staking_info.reward_vaults_bump)]
    pub reward_token_vaults: Account<'info, TokenAccount>,
    #[account(mut, token::mint = staking_info.reward_token_mint)]
    pub sender_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut, seeds = [b"staking_info"], bump = staking_info.bump)]
    pub staking_info: Account<'info, StakingInfo>,
    #[account(init_if_needed, payer = signer, seeds = [b"user_stake_info", signer.key().as_ref()], bump, space = 8 + UserStakeInfo::MAX_SIZE)]
    pub user_stake_info: Account<'info, UserStakeInfo>,
    #[account(mut)]
    pub staking_token_mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"staking_token_vaults", staking_token_mint.key().as_ref()], bump = staking_info.staking_vaults_bump)]
    pub staking_token_vaults: Account<'info, TokenAccount>,
    #[account(mut, token::mint = staking_info.staking_token_mint, token::authority = signer.key())]
    pub sender_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut, seeds = [b"staking_info"], bump = staking_info.bump)]
    pub staking_info: Account<'info, StakingInfo>,
    #[account(mut, seeds = [b"user_stake_info", signer.key().as_ref()], bump)]
    pub user_stake_info: Account<'info, UserStakeInfo>,
    #[account(mut)]
    pub staking_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub reward_token_mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"staking_token_vaults", staking_token_mint.key().as_ref()], bump = staking_info.staking_vaults_bump)]
    pub staking_token_vaults: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"reward_token_vaults", reward_token_mint.key().as_ref()], bump = staking_info.reward_vaults_bump)]
    pub reward_token_vaults: Account<'info, TokenAccount>,
    #[account(mut, token::mint = staking_info.staking_token_mint, token::authority = signer.key())]
    pub recipient_staking_token: Account<'info, TokenAccount>,
    #[account(mut, token::mint = staking_info.reward_token_mint, token::authority = signer.key())]
    pub recipient_reward_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakingInfo {
    pub lock_period: [u64; 4],
    pub reward_rate: [u64; 4],
    pub staking_token_mint: Pubkey,
    pub reward_token_mint: Pubkey,
    pub owner: Pubkey,
    pub total_staked: u64,

    bump: u8,
    staking_vaults_bump: u8,
    reward_vaults_bump: u8,
}

impl StakingInfo {
    pub const MAX_SIZE: usize = 8 * 4 + 8 * 4 + 32 + 32 + 32 + 8 + 1 + 1 + 1;
}

#[account]
pub struct UserStakeInfo {
    pub amount: [u64; 4],
    pub staked_time: [u64; 4],
    pub claimed_time: [u64; 4],
    pub claimed_amount: [u64; 4],
    pub pending_reward: [u64; 4],
}

impl UserStakeInfo {
    pub const MAX_SIZE: usize = 8 * 4 * 5;
}

#[error_code]
pub enum StakingError {
    #[msg("NOT_OWNER")]
    NotOwner,
    #[msg("INSUFFICIENT BALANCE")]
    InsufficientBalance,
    #[msg("IN LOCK PERIOD")]
    Locked,
}
