use anchor_lang::prelude::*;
use solana_program::pubkey;
use solana_program::system_instruction;

declare_id!("GdwhYEQqsQUBvMMFtpZSKmBaWHtfKoNh24Pf91Qoo5a6");

#[program]
pub mod tower_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let clock = Clock::get()?;
        let time_stamp = clock.unix_timestamp; // current timestamp
        let game_state = &mut ctx.accounts.game_state;
        if game_state.is_init {
            return err!(ErrorCode::GameAlreadyInit);
        }
        game_state.tournament_id = 1;
        game_state.tournament_start_at = time_stamp;
        game_state.server_address = *ctx.accounts.server_address.key;
        game_state.is_init = true;
        Ok(())
    }

    pub fn participate_tournament(
        ctx: Context<ParticipateTournament>,
        tournament_id: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let time_stamp = clock.unix_timestamp;
        let mut tournament_start_at = ctx.accounts.game_state.tournament_start_at;
        let mut cur_tournament_id = ctx.accounts.game_state.tournament_id;
        if time_stamp - tournament_start_at >= 24 * 3600 {
            cur_tournament_id += 1;
            tournament_start_at = time_stamp;
        }
        if cur_tournament_id != tournament_id {
            return err!(ErrorCode::TournamentNotOngoing);
        }
        ctx.accounts.game_state.tournament_id = cur_tournament_id;
        ctx.accounts.game_state.tournament_start_at = tournament_start_at;
        let user_tournament_account = &mut ctx.accounts.user_tournament_account;
        user_tournament_account.health = 3;
        user_tournament_account.sessions = 1;

        Ok(())
    }

    pub fn tap(ctx: Context<Tap>, reward_amount: u64) -> Result<()> {
        let clock = Clock::get()?;
        let time_stamp = clock.unix_timestamp;
        let game_state = &mut ctx.accounts.game_state;
        let tournament_account = &mut ctx.accounts.tournament_account;
        if time_stamp - game_state.tournament_start_at >= 24 * 3600 {
            return err!(ErrorCode::TournamentFinished);
        }

        let base_fee: u64 = 1;
        let amount = base_fee.checked_mul(10u64.pow(6)).unwrap();
        let transfer_instruction =
            system_instruction::transfer(&ctx.accounts.user.key, &ctx.accounts.server.key, amount);

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.server.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        let user_tournament_account = &mut ctx.accounts.user_tournament_account;
        if reward_amount == 0 {
            user_tournament_account.health -= 1;
            user_tournament_account.sessions += 1;
        }
        user_tournament_account.rewards_earned += reward_amount;
        if tournament_account.top_score > user_tournament_account.rewards_earned {
            tournament_account.top_score = user_tournament_account.rewards_earned;
            tournament_account.top_player = ctx.accounts.user.key();
        }

        user_tournament_account.taps += 1;
        Ok(())
    }

    pub fn buy_chance(ctx: Context<BuyChance>) -> Result<()> {
        let clock = Clock::get()?;
        let time_stamp = clock.unix_timestamp;
        let game_state = &ctx.accounts.game_state;
        if time_stamp - game_state.tournament_start_at >= 24 * 3600 {
            return err!(ErrorCode::TournamentFinished);
        }
        let user_tournament_account = &mut ctx.accounts.user_tournament_account;
        //let mut fee_multiplyer = 1;
        //let mut base_fee: i32 = 1;
        /*if user_tournament_account.sessions > 3 {
            fee_multiplyer = (user_tournament_account.sessions - 3) * 2;
        }
        base_fee = base_fee * fee_multiplyer;*/
        let tournament_account = &mut ctx.accounts.tournament_account;
        let base_fee: u64 = 5;
        let amount = base_fee.checked_mul(10u64.pow(8)).unwrap();
        // Create the transfer instruction
        let transfer_instruction = system_instruction::transfer(
            &ctx.accounts.user.key,
            &ctx.accounts.server_wallet.key,
            amount,
        );

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.server_wallet.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
        tournament_account.prize_pool += amount;
        user_tournament_account.health += 1;
        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>, _tournament_id: u64) -> Result<()> {
        let tournament_account = &mut ctx.accounts.tournament_account;
        if tournament_account.is_rewarded {
            return err!(ErrorCode::RewardClaimed);
        }
        if tournament_account.top_player != *ctx.accounts.user.key {
            return err!(ErrorCode::NotWinner);
        }
        let amount = tournament_account
            .prize_pool
            .checked_div(100)
            .unwrap()
            .checked_mul(70)
            .unwrap();
        // Create the transfer instruction
        let transfer_instruction =
            system_instruction::transfer(&ctx.accounts.server.key, &ctx.accounts.user.key, amount);

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                ctx.accounts.server.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
        tournament_account.is_rewarded = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 1 + 8 + 8, seeds = [], bump)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub server_address: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(tournament_id: u64)]
pub struct ParticipateTournament<'info> {
    #[account(mut, seeds = [], bump)]
    pub game_state: Account<'info, GameState>,
    #[account(init_if_needed, payer = user, space = 8 + 32 + 8 + 8 + 1, seeds = [b"tourname", &tournament_id.to_le_bytes()], bump)]
    pub tournament_account: Account<'info, TournamentState>,
    #[account(init, payer = user, space = 8 + 8 + 8 + 8 + 8, seeds = [b"user_account", user.key().as_ref(), &tournament_id.to_le_bytes()], bump)]
    pub user_tournament_account: Account<'info, UserTournamentState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_tournament_id: u64)]
pub struct ClaimReward<'info> {
    #[account(mut, seeds = [], bump)]
    pub game_state: Account<'info, GameState>,
    #[account(mut, seeds = [b"tourname", &_tournament_id.to_le_bytes()], bump)]
    pub tournament_account: Account<'info, TournamentState>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, address=game_state.server_address)]
    pub server: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Tap<'info> {
    #[account(mut, seeds = [], bump)]
    pub game_state: Account<'info, GameState>,
    #[account(mut, seeds = [b"user_account", user.key().as_ref(), &game_state.tournament_id.to_le_bytes()], bump)]
    pub user_tournament_account: Account<'info, UserTournamentState>,
    #[account(mut, seeds = [b"tourname", &game_state.tournament_id.to_le_bytes()], bump)]
    pub tournament_account: Account<'info, TournamentState>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, address=game_state.server_address)]
    pub server: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyChance<'info> {
    #[account(mut, seeds = [], bump)]
    pub game_state: Account<'info, GameState>,
    #[account(mut, seeds = [b"user_account", user.key().as_ref(), &game_state.tournament_id.to_le_bytes()], bump)]
    pub user_tournament_account: Account<'info, UserTournamentState>,
    #[account(mut, seeds = [b"tourname", &game_state.tournament_id.to_le_bytes()], bump)]
    pub tournament_account: Account<'info, TournamentState>,
    #[account(mut, address=game_state.server_address)]
    pub server_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserTournamentState {
    pub health: u64,
    pub rewards_earned: u64,
    pub sessions: u64,
    pub taps: u64,
}

#[account]
pub struct GameState {
    pub is_init: bool,
    pub server_address: Pubkey,
    pub tournament_id: u64,
    pub tournament_start_at: i64,
}

#[account]
pub struct TournamentState {
    pub top_player: Pubkey,
    pub top_score: u64,
    pub is_rewarded: bool,
    pub prize_pool: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Game already init!")]
    GameAlreadyInit,
    #[msg("Given tournament is not currently on-going")]
    TournamentNotOngoing,
    #[msg("Tournament already finished")]
    TournamentFinished,
    #[msg("Not enough chances left to play this tournament buy it to continue")]
    NotEnoughChances,
    #[msg("Reward already claimed!!")]
    RewardClaimed,
    #[msg("You are not a winner!")]
    NotWinner,
}
