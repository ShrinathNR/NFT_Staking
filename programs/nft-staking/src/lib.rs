pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HsSHFPV1v4NPryx2DvjmMy8Z8BjWb2ucWJCmqA2N6YAs");

#[program]
pub mod nft_staking {
    use super::*;

    
}
