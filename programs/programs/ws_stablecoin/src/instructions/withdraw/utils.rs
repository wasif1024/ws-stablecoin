use crate::constants::SOL_SEED;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token_2022::{burn, Burn};
use anchor_spl::token_interface::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};
pub fn withdraw_sol<'info>(
    bump: u8,
    depositor_key: Pubkey,
    system_program: &Program<'info, System>,
    amount: u64,
    from: &SystemAccount<'info>,
    to: &AccountInfo<'info>,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SOL_SEED, depositor_key.as_ref(), &[bump]]];
    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
    //Ok(())
}
pub fn burn_tokens<'info>(
    mint_account: &InterfaceAccount<'info, Mint>,
    token_program: &Program<'info, Token2022>,
    amount: u64,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    authority: &Signer<'info>,
) -> Result<()> {
    burn(
        CpiContext::new(
            token_program.to_account_info(),
            Burn {
                mint: mint_account.to_account_info(),
                from: token_account.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        amount,
    )
    //Ok(())
}
