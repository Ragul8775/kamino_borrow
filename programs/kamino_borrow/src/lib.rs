use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_spl::token::{Token, TokenAccount};

declare_id!("3DSox9PrsL3PTJDuNq5p4kCr9TkfVFN8dAhMffJ6DiBr");

#[program]
pub mod kamino_borrow {

    use super::*;
    use anchor_lang::solana_program::program::invoke;

    pub fn execute_kamino_borrow(
        ctx: Context<ExecuteKaminoBorrow>,
        liquidity_amount: u64,
    ) -> Result<()> {
        /*  // Set the compute budget
               invoke(
                   &compute_budget_instruction(200000),
                   &[ctx.accounts.fee_payer.to_account_info()],
               )?;

               invoke(
                   &compute_budget_instruction(231847),
                   &[ctx.accounts.fee_payer.to_account_info()],
               )?;
        */
        // Refresh the reserve for the collateral
        invoke(
            &refresh_reserve_instruction(&ctx.accounts.kamino_program.key(), &ctx.accounts),
            &[
                ctx.accounts.reserve_collateral.to_account_info(),
                ctx.accounts.lending_market.to_account_info(),
                ctx.accounts.pyth_oracle.to_account_info(),
                ctx.accounts.switchboard_price_oracle.to_account_info(),
                ctx.accounts.switchboard_twap_oracle.to_account_info(),
                ctx.accounts.scope_prices.to_account_info(),
            ],
        )?;

        //refresh reserve for the borrow
        invoke(
            &refresh_reserve_instruction(&ctx.accounts.kamino_program.key(), &ctx.accounts),
            &[
                ctx.accounts.reserve_borrow.to_account_info(),
                ctx.accounts.lending_market.to_account_info(),
                ctx.accounts.pyth_oracle.to_account_info(),
                ctx.accounts.switchboard_price_oracle.to_account_info(),
                ctx.accounts.switchboard_twap_oracle.to_account_info(),
                ctx.accounts.scope_prices.to_account_info(),
            ],
        )?;

        // Refresh the obligation

        invoke(
            &refresh_obligation_instruction(&ctx.accounts.kamino_program.key(), &ctx.accounts),
            &[
                ctx.accounts.lending_market.to_account_info(),
                ctx.accounts.obligation.to_account_info(),
            ],
        )?;
        // Borrow liquidity
        invoke(
            &borrow_liquidity_instruction(
                &ctx.accounts.kamino_program.key(),
                &ctx.accounts,
                liquidity_amount,
            ),
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.obligation.to_account_info(),
                ctx.accounts.lending_market.to_account_info(),
                ctx.accounts.lending_market_authority.to_account_info(),
                ctx.accounts.borrow_reserve.to_account_info(),
                ctx.accounts.borrow_reserve_liquidity_mint.to_account_info(),
                ctx.accounts.reserve_source_liquidity.to_account_info(),
                ctx.accounts
                    .borrow_reserve_liquidity_fee_reciever
                    .to_account_info(),
                ctx.accounts.user_destination_liquidity.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.instruction_sysvar.to_account_info(),
            ],
        )?;
        Ok(())
        //
    }
}

#[derive(Accounts)]

pub struct ExecuteKaminoBorrow<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub fee_payer: Signer<'info>,
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_borrow: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub borrow_reserve: AccountInfo<'info>,
    #[account(mut)]
    pub borrow_reserve_liquidity_mint: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_source_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub borrow_reserve_liquidity_fee_reciever: AccountInfo<'info>,
    #[account(mut)]
    pub user_destination_liquidity: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pyth_oracle: AccountInfo<'info>,
    #[account(mut)]
    pub switchboard_price_oracle: AccountInfo<'info>,
    #[account(mut)]
    pub switchboard_twap_oracle: AccountInfo<'info>,
    #[account(mut)]
    pub scope_prices: AccountInfo<'info>,
    #[account(mut)]
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub instruction_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub kamino_program: AccountInfo<'info>,
}

/* fn compute_budget_instruction(units: u32) -> Instruction {
    Instruction {
        program_id: solana_program::compute_budget::ID,
        accounts: vec![],
        data: solana_program::compute_budget::ComputerBudgetInstruction::SetComputeUnitLimit {
            units,
        }
        .pack(),
    }
}
fn compute_budget_price_instruction(micro_lamports: u64) -> Instruction {
    Instruction {
        program_id: solana_program::compute_budget::ID,
        accounts: vec![],
        data: solana_program::compute_budget::ComputeBudgetInstruction::SetComputeUnitPrice {
            micro_lamports,
        }
        .pack(),
    }
}
 */
fn refresh_reserve_instruction(
    kamino_program_id: &Pubkey,
    accounts: &ExecuteKaminoBorrow,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts.reserve_collateral.key(), false),
            AccountMeta::new(accounts.lending_market.key(), false),
            AccountMeta::new(accounts.pyth_oracle.key(), false),
            AccountMeta::new(accounts.switchboard_price_oracle.key(), false),
            AccountMeta::new(accounts.switchboard_twap_oracle.key(), false),
            AccountMeta::new(accounts.scope_prices.key(), false),
        ],
        data: vec![0],
    }
}

fn refresh_obligation_instruction(
    kamino_program_id: &Pubkey,
    accounts: &ExecuteKaminoBorrow,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts.lending_market.key(), false),
            AccountMeta::new(accounts.obligation.key(), false),
        ],
        data: vec![1],
    }
}

fn borrow_liquidity_instruction(
    kamino_program_id: &Pubkey,
    accounts: &ExecuteKaminoBorrow,
    liquidity_amount: u64,
) -> Instruction {
    let mut data = vec![2];
    data.extend_from_slice(&liquidity_amount.to_le_bytes());
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(accounts.owner.key(), true),
            AccountMeta::new(accounts.obligation.key(), false),
            AccountMeta::new(accounts.lending_market.key(), false),
            AccountMeta::new(accounts.lending_market_authority.key(), false),
            AccountMeta::new(accounts.borrow_reserve.key(), false),
            AccountMeta::new(accounts.borrow_reserve_liquidity_mint.key(), false),
            AccountMeta::new(accounts.reserve_source_liquidity.key(), false),
            AccountMeta::new(accounts.borrow_reserve_liquidity_fee_reciever.key(), false),
            AccountMeta::new(accounts.user_destination_liquidity.key(), false),
            AccountMeta::new_readonly(accounts.token_program.key(), false),
            AccountMeta::new_readonly(accounts.instruction_sysvar.key(), false),
        ],
        data,
    }
}
