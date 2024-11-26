use anchor_lang::prelude::*;

pub mod create_post;
pub mod edit_post;
pub mod delete_post;

use create_post::*;
use edit_post::*;
use delete_post::*;

declare_id!("FHMvSSY4Ne9j7Zgjcus5VBCtW2yykTQfnTXxiuVYbauX");

#[program]
pub mod onchain_blog {
    use super::*;

    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String, timestamp: u64) -> Result<()> {
        create_post::create_post(ctx, title, content, timestamp)
    }

    pub fn edit_post(ctx: Context<EditPost>, title: String, content: String) -> Result<()> {
        edit_post::edit_post(ctx, title, content)
    }

    pub fn toggle_publish(ctx: Context<TogglePublish>) -> Result<()> {
        edit_post::toggle_publish(ctx)
    }

    pub fn delete_post(ctx: Context<DeletePost>) -> Result<()> {
        delete_post::delete_post(ctx)
    }
}

#[account]
pub struct Post {
    pub author: Pubkey,         
    pub title: String,      
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub is_published: bool,
}

impl Post {
    pub const MAX_SIZE: usize = 8 + 32 + 256 + 1024 + 8 + 8 + 1; 
}
