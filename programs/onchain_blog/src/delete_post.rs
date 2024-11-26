use anchor_lang::prelude::*;

use crate::Post;

#[derive(Accounts)]
pub struct DeletePost<'info> {
    #[account(mut, has_one = author, close = author)]
    pub post: Account<'info, Post>,
    pub author: Signer<'info>,
}

pub fn delete_post(ctx: Context<DeletePost>) -> Result<()> {
    msg!("Post by {} deleted. - Psolite", ctx.accounts.author.key());
    Ok(())
}