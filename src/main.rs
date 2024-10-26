use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{error::Error, fs::File, io::{BufWriter, Write}, str::FromStr};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc_url: String,

    #[arg(short, long)]
    program_id: String,
}

pub struct SolanaIndexer {
    client: RpcClient,
}

impl SolanaIndexer {
    pub fn new(rpc_url: &str) -> Self {
        let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
        SolanaIndexer { client }
    }

    pub async fn get_recent_block(&self) -> Result<u64, Box<dyn Error>> {
        let slot = self.client.get_slot()?;
        Ok(slot)
    }

    pub async fn fetch_program_accounts(&self, program_id: &str) -> Result<(), Box<dyn Error>> {
        let pubkey = Pubkey::from_str(program_id)?;

        let accounts = self.client.get_program_accounts(&pubkey)?;
        for (pubkey, account) in accounts.iter() {
            println!("Account: {:?}, Data: {:?}", pubkey, account.data);
        }

        Ok(())
    }

    pub fn save_data_to_file(&self, data: &str, filename: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(data.as_bytes())?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let indexer = SolanaIndexer::new(&args.rpc_url);

    if let Err(e) = indexer.fetch_program_accounts(&args.program_id).await {
        eprintln!("Error fetching accounts: {}", e);
    }
}
