use crate::SEED_SOL_ACCOUNT;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    token_2022::{burn, Burn},
    token_interface::{Mint, Token2022, TokenAccount},
};

pub fn withdraw_sol<'info>(
    bump: u8,
    depositor_key: &Pubkey,
    system_program: &Program<'info, System>,
    to: &AccountInfo<'info>,
    from: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_SOL_ACCOUNT, depositor_key.as_ref(), &[bump]]];
    let cpi_context = CpiContext::new_with_signer(
        system_program.to_account_info(),
        Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
        },
        signer_seeds,
    );
    transfer(cpi_context, amount)
}

pub fn burn_tokens<'info>(
    amount: u64,
    token_program: &Program<'info, Token2022>,
    mint: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    authority: &Signer<'info>,
) -> Result<()> {
    let cpi_context = CpiContext::new(
        token_program.to_account_info(),
        Burn {
            mint: mint.to_account_info(),
            from: token_account.to_account_info(),
            authority: authority.to_account_info(),
        },
    );
    burn(cpi_context, amount)
}
