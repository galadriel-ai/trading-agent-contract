use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::hash;

declare_id!("H5CtT4c2mmkcwSAWc4nWTFXjD9bQGgcag57WEwrucLwQ");

const DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod solana_attestation_contract {

    use super::*;

    // Initialize the agent data and set the admin to the signer
    pub fn initialize_agent(ctx: Context<InitializeAgent>) -> Result<()> {
        let agent_data = &mut ctx.accounts.agent_data;

        agent_data.admin = ctx.accounts.signer.key();

        Ok(())
    }

    pub fn request_trading(
        ctx: Context<RequestTrading>, 
        args: RequestTradingArgs
    ) -> Result<()> {
        let agent_data = &ctx.accounts.agent_data;

        let signer = ctx.accounts.signer.key();

        // Concatenate the trading data and calculate the hash
        let mut data = Vec::new();
        data.extend_from_slice(&args.trade_operation.to_bytes());
        data.extend_from_slice(&args.from_token.to_bytes());
        data.extend_from_slice(&args.to_token.to_bytes());
        data.extend_from_slice(&args.amount.to_le_bytes());
        data.extend_from_slice(&args.nonce.to_le_bytes());

        let _hashed_data = hash(&data);

        // Verify if the signature is matched with the hashed data and the TEE key
        //let signature = &args.signature;
        //let tee_key = ed25519_dalek::PublicKey::from_bytes(&agent_data.tee_key.to_bytes()).map_err(|_| Errors::InvalidPublicKey)?;
        //let signature = ed25519_dalek::Signature::from_bytes(signature).map_err(|_| Errors::InvalidSignature)?;
        //require!(tee_key.verify(&hashed_data.to_bytes(), &signature).is_ok(), Errors::InvalidSignature);

        // TODO Perform the trading operation

        Ok(())
    }

    // Update the TEE key of the agent
    pub fn update_tee_key(
        ctx: Context<UpdateTeeKey>,
        args: UpdateTeeKeyArgs
    ) -> Result<()> {
        let agent_data = &mut ctx.accounts.agent_data;
        let signer = ctx.accounts.signer.key();

        require!(agent_data.admin == signer, Errors::UnauthorizedSigner);

        agent_data.tee_key = args.tee_key;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAgent<'info> {
    #[account(
        init, 
        payer = signer, 
        space = DISCRIMINATOR_SIZE + AgentData::INIT_SPACE,
        seeds = [b"agent", signer.key().as_ref()],
        bump
    )]
    pub agent_data: Account<'info, AgentData>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RequestTradingArgs {
    pub agent_admin: Pubkey,
    pub trade_operation: TradingOperation,
    pub from_token: Pubkey,
    pub to_token: Pubkey,
    pub amount: u64,
    pub nonce: u64,
    pub signature: [u8; 64],
}

#[derive(Accounts)]
#[instruction(args: RequestTradingArgs)]
pub struct RequestTrading<'info> {
    #[account(
        seeds = [b"agent", args.agent_admin.as_ref()],
        bump
    )]
    pub agent_data: Account<'info, AgentData>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateTeeKeyArgs {
    pub tee_key: Pubkey,
}

#[derive(Accounts)]
pub struct UpdateTeeKey<'info> {
    #[account(
        mut,
        seeds = [b"agent", signer.key().as_ref()],
        bump
    )]
    pub agent_data: Account<'info, AgentData>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct AgentData {
    pub admin: Pubkey,
    pub tee_key: Pubkey
}


#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum TradingOperation {
    Buy,
    Sell,
}

impl TradingOperation {
    pub fn to_bytes(&self) -> [u8; 1] {
        match self {
            TradingOperation::Buy => [0],
            TradingOperation::Sell => [1],
        }
    }
}

#[error_code]
pub enum Errors {
    #[msg("Unauthorized signer")]
    UnauthorizedSigner,
    #[msg("Invalid signature")]
    InvalidSignature,
    #[msg("Invalid public key")]
    InvalidPublicKey,
}