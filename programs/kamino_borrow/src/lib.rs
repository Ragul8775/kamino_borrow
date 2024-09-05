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
        // Refresh the reserve for the collateral
        invoke(
            &refresh_reserve_instruction(
                &ctx.accounts.kamino_program.key(),
                &ctx.remaining_accounts,
            ),
            &[
                ctx.remaining_accounts[0].clone(),
                ctx.remaining_accounts[1].clone(),
                ctx.remaining_accounts[2].clone(),
                ctx.remaining_accounts[3].clone(),
                ctx.remaining_accounts[4].clone(),
                ctx.remaining_accounts[5].clone(),
            ],
        )?;

        //refresh reserve for the borrow
        invoke(
            &refresh_reserve_instruction(
                &ctx.accounts.kamino_program.key(),
                &ctx.remaining_accounts,
            ),
            &[
                ctx.remaining_accounts[6].clone(),
                ctx.remaining_accounts[1].clone(),
                ctx.remaining_accounts[2].clone(),
                ctx.remaining_accounts[3].clone(),
                ctx.remaining_accounts[4].clone(),
                ctx.remaining_accounts[5].clone(),
            ],
        )?;

        // Refresh the obligation
        invoke(
            &refresh_obligation_instruction(
                &ctx.accounts.kamino_program.key(),
                &ctx.remaining_accounts,
            ),
            &[
                ctx.remaining_accounts[1].clone(),
                ctx.remaining_accounts[7].clone(),
            ],
        )?;

        // Borrow liquidity
        invoke(
            &borrow_liquidity_instruction(
                &ctx.accounts.kamino_program.key(),
                ctx.remaining_accounts,
                liquidity_amount,
            ),
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.remaining_accounts[7].clone(),
                ctx.remaining_accounts[1].clone(),
                ctx.remaining_accounts[8].clone(),
                ctx.remaining_accounts[9].clone(),
                ctx.remaining_accounts[10].clone(),
                ctx.remaining_accounts[11].clone(),
                ctx.remaining_accounts[12].clone(),
                ctx.accounts.user_destination_liquidity.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.instruction_sysvar.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteKaminoBorrow<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub fee_payer: Signer<'info>,
    #[account(mut)]
    pub user_destination_liquidity: Account<'info, TokenAccount>,
    /// CHECK: This is safe because it is the Token Program required for SPL tokens
    #[account(mut)]
    pub token_program: AccountInfo<'info>,
    /// CHECK: This is safe because it's the instruction sysvar for the program
    #[account(mut)]
    pub instruction_sysvar: AccountInfo<'info>,
    /// CHECK: This is safe because we are interacting with the Kamino program
    #[account(mut)]
    pub kamino_program: AccountInfo<'info>,
}

fn refresh_reserve_instruction(
    kamino_program_id: &Pubkey,
    remaining_accounts: &[AccountInfo],
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(*remaining_accounts[0].key, false),
            AccountMeta::new(*remaining_accounts[1].key, false),
            AccountMeta::new(*remaining_accounts[2].key, false),
            AccountMeta::new(*remaining_accounts[3].key, false),
            AccountMeta::new(*remaining_accounts[4].key, false),
            AccountMeta::new(*remaining_accounts[5].key, false),
        ],
        data: vec![0],
    }
}

fn refresh_obligation_instruction(
    kamino_program_id: &Pubkey,
    remaining_accounts: &[AccountInfo],
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(*remaining_accounts[1].key, false),
            AccountMeta::new(*remaining_accounts[7].key, false),
        ],
        data: vec![1],
    }
}

fn borrow_liquidity_instruction(
    kamino_program_id: &Pubkey,
    remaining_accounts: &[AccountInfo],
    liquidity_amount: u64,
) -> Instruction {
    let mut data = vec![2];
    data.extend_from_slice(&liquidity_amount.to_le_bytes());
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(*remaining_accounts[0].key, true),
            AccountMeta::new(*remaining_accounts[7].key, false),
            AccountMeta::new(*remaining_accounts[1].key, false),
            AccountMeta::new(*remaining_accounts[8].key, false),
            AccountMeta::new(*remaining_accounts[9].key, false),
            AccountMeta::new(*remaining_accounts[10].key, false),
            AccountMeta::new(*remaining_accounts[11].key, false),
            AccountMeta::new(*remaining_accounts[12].key, false),
            AccountMeta::new(*remaining_accounts[13].key, false),
            AccountMeta::new_readonly(*remaining_accounts[14].key, false),
            AccountMeta::new_readonly(*remaining_accounts[15].key, false),
        ],
        data,
    }
}
