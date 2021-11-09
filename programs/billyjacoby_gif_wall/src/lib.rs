use anchor_lang::prelude::*;

declare_id!("LncxPiPXCrxhfoyASy289iBazYah37JT5n6HaUHGmzc");

#[program]
pub mod billyjacoby_gif_wall {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // Get a reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String, user_name: String) -> ProgramResult {
        // Get a reference to the account and increment the total gifs
        let base_account = &mut ctx.accounts.base_account;

        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            user_name: user_name.to_string(),
        };

        // Add it to the gif_list vector.
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

// Attach certain variables to the StartStuffOff context
#[derive(Accounts)]
pub struct StartStuffOff<'info>{
    #[account(init, payer=user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}



// Create a custom struct to work with
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub user_name: String,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a vector of type itemSTruct to the account.
    pub gif_list: Vec<ItemStruct>,
}