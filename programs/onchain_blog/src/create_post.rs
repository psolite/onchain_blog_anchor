use anchor_lang::prelude::*;

use crate::Post;

#[derive(Accounts)]
#[instruction(title: String, content: String, timestamp: u64)]
pub struct CreatePost<'info> {
    #[account(
        init, 
        payer = author, 
        space = Post::MAX_SIZE,
        seeds = [b"psolite", author.key().as_ref(), &timestamp.to_le_bytes()],
        bump
    )]
    pub post: Account<'info, Post>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_post(ctx: Context<CreatePost>, title: String, content: String, _timestamp: u64) -> Result<()> {
    let post = &mut ctx.accounts.post;
    let clock = Clock::get()?;
    
    post.author = ctx.accounts.author.key();
    post.title = title;
    post.content = content;
    post.created_at = clock.unix_timestamp;
    post.updated_at = clock.unix_timestamp;
    post.is_published = true;

    msg!("Post created. Title: {} - Psolite", post.title);
    Ok(())
}