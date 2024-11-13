use anchor_lang::prelude::*;
use state::*;
mod state;
use constants::*;
mod constants;
use instructions::*;
mod instructions;
pub use error::*;

mod error;
declare_id!("EB51MiGRKfh7nwPUK1LWzm72aSkSLizpSPj9Sfqiq2jb");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initoalize_config(ctx)
    }
    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }
}
