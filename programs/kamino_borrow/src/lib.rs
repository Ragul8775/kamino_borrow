use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;

declare_id!("3DSox9PrsL3PTJDuNq5p4kCr9TkfVFN8dAhMffJ6DiBr");

#[program]
pub mod kamino_borrow {
    use super::*;
    use anchor_lang::solana_program::program::invoke;

    pub fn execute_kamino_borrow(
        ctx: Context<ExecuteKaminoOperations>,
        data: Vec<Vec<u8>>,
    ) -> Result<()> {
        invoke(
            &initiate_ixns(
                &ctx.accounts.kamino_program.key(),
                &ctx.remaining_accounts,
                data[0].clone(),
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

        // Refresh the reserve for the collateral
        invoke(
            &&refresh_reserve_instruction_collateral(
                &ctx.accounts.kamino_program.key(),
                &ctx.remaining_accounts,
                data[1].clone(),
            ),
            &[
                ctx.remaining_accounts[6].clone(),
                ctx.remaining_accounts[7].clone(),
                ctx.remaining_accounts[8].clone(),
                ctx.remaining_accounts[9].clone(),
                ctx.remaining_accounts[10].clone(),
                ctx.remaining_accounts[11].clone(),
            ],
        )?;

        //refresh reserve for the borrow
        invoke(
            &refresh_reserve_instruction(
                &ctx.accounts.kamino_program.key(),
                &ctx.remaining_accounts,
                data[2].clone(),
            ),
            &[
                ctx.remaining_accounts[12].clone(),
                ctx.remaining_accounts[13].clone(),
                ctx.remaining_accounts[14].clone(),
                ctx.remaining_accounts[15].clone(),
                ctx.remaining_accounts[16].clone(),
                ctx.remaining_accounts[17].clone(),
            ],
        )?;

        // Refresh the obligation
        invoke(
            &refresh_obligation_instruction(
                &ctx.accounts.kamino_program.key(),
                &ctx.remaining_accounts,
                data[3].clone(),
            ),
            &[
                ctx.remaining_accounts[18].clone(),
                ctx.remaining_accounts[19].clone(),
            ],
        )?;

        // Borrow liquidity
        invoke(
            &borrow_liquidity_instruction(
                &ctx.accounts.kamino_program.key(),
                ctx.remaining_accounts,
                data[4].clone(),
            ),
            &[
                ctx.remaining_accounts[20].clone(),
                ctx.remaining_accounts[21].clone(),
                ctx.remaining_accounts[22].clone(),
                ctx.remaining_accounts[23].clone(),
                ctx.remaining_accounts[24].clone(),
                ctx.remaining_accounts[25].clone(),
                ctx.remaining_accounts[26].clone(),
                ctx.remaining_accounts[27].clone(),
                ctx.remaining_accounts[28].clone(),
                ctx.remaining_accounts[29].clone(),
                ctx.remaining_accounts[30].clone(),
            ],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteKaminoOperations<'info> {
    /// CHECK: This account is not directly used by the program and is passed to CPI. Safe because we trust the external program's account validation.
    pub kamino_program: AccountInfo<'info>,
}

fn initiate_ixns(
    kamino_program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new_readonly(accounts[0].key(), true),
            AccountMeta::new(accounts[1].key(), true),
            AccountMeta::new(accounts[2].key(), false),
            AccountMeta::new_readonly(accounts[3].key(), false),
            AccountMeta::new_readonly(accounts[4].key(), false),
            AccountMeta::new_readonly(accounts[5].key(), false),
        ],
        data,
    }
}

fn refresh_reserve_instruction(
    kamino_program_id: &Pubkey,
    remaining_accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new_readonly(remaining_accounts[6].key(), true),
            AccountMeta::new(*remaining_accounts[7].key, false),
            AccountMeta::new(*remaining_accounts[8].key, false),
            AccountMeta::new(*remaining_accounts[9].key, false),
            AccountMeta::new(*remaining_accounts[10].key, false),
            AccountMeta::new(*remaining_accounts[11].key, false),
            AccountMeta::new(*remaining_accounts[12].key, false),
        ],
        data,
    }
}
fn refresh_reserve_instruction_collateral(
    kamino_program_id: &Pubkey,
    remaining_accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new_readonly(remaining_accounts[13].key(), true),
            AccountMeta::new(*remaining_accounts[14].key, false),
            AccountMeta::new(*remaining_accounts[15].key, false),
            AccountMeta::new(*remaining_accounts[16].key, false),
            AccountMeta::new(*remaining_accounts[17].key, false),
            AccountMeta::new(*remaining_accounts[18].key, false),
            AccountMeta::new(*remaining_accounts[19].key, false),
        ],
        data,
    }
}

fn refresh_obligation_instruction(
    kamino_program_id: &Pubkey,
    remaining_accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(*remaining_accounts[20].key, false),
            AccountMeta::new(*remaining_accounts[21].key, false),
        ],
        data,
    }
}

fn borrow_liquidity_instruction(
    kamino_program_id: &Pubkey,
    remaining_accounts: &[AccountInfo],
    data: Vec<u8>,
) -> Instruction {
    Instruction {
        program_id: *kamino_program_id,
        accounts: vec![
            AccountMeta::new(*remaining_accounts[22].key, true),
            AccountMeta::new(*remaining_accounts[23].key, false),
            AccountMeta::new(*remaining_accounts[24].key, false),
            AccountMeta::new(*remaining_accounts[25].key, false),
            AccountMeta::new(*remaining_accounts[26].key, false),
            AccountMeta::new(*remaining_accounts[27].key, false),
            AccountMeta::new(*remaining_accounts[28].key, false),
            AccountMeta::new(*remaining_accounts[29].key, false),
            AccountMeta::new(*remaining_accounts[30].key, false),
            AccountMeta::new_readonly(*remaining_accounts[31].key, false),
            AccountMeta::new_readonly(*remaining_accounts[32].key, false),
        ],
        data,
    }
}
