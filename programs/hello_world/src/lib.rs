use anchor_lang::prelude::*;

declare_id!("DH8ue4uDedp1bDJ7XuuZP87boAqU7mhLghjTDqh65Z9k");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
