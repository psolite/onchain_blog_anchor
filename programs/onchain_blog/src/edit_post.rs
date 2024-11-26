use anchor_lang::prelude::*;

use crate::Post;

#[derive(Accounts)]
pub struct EditPost<'info> {
    #[account(mut, has_one = author)]
    pub post: Account<'info, Post>,
    pub author: Signer<'info>,
}

pub fn edit_post(ctx: Context<EditPost>, new_title: String, new_content: String) -> Result<()> {
    let post = &mut ctx.accounts.post;
    let clock = Clock::get()?;
    
    post.title = new_title;
    post.content = new_content;
    post.updated_at = clock.unix_timestamp;

    msg!("Post updated. Current title: {} - Psolite", post.title);

    Ok(())
}

#[derive(Accounts)]
pub struct TogglePublish<'info> {
    #[account(mut, has_one = author)]
    pub post: Account<'info, Post>,
    pub author: Signer<'info>,
}

pub fn toggle_publish(ctx: Context<TogglePublish>) -> Result<()> {
    let post = &mut ctx.accounts.post;
    if post.is_published {
        post.is_published = false;
    } else {
        post.is_published = true;
    }
    

    Ok(())
}