use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{error::Error, fs::File, io::{BufWriter, Write}, str::FromStr};
use clap::{Parser, Subcommand};
use serde_json::json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc_url: String,

    #[arg(short, long)]
    program_id: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    FetchAccounts,
    GetBalance { account_pubkey: String },
    GetTransactionCount,
    ExportAccountsJson { filename: String },
}

pub struct SolanaIndexer {
    client: RpcClient,
}

impl SolanaIndexer {
    pub fn new(rpc_url: &str) -> Self {
        let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
        SolanaIndexer { client }
    }

    pub async fn fetch_program_accounts(&self, program_id: &str) -> Result<(), Box<dyn Error>> {
        let pubkey = Pubkey::from_str(program_id)?;

        let accounts = self.client.get_program_accounts(&pubkey)?;
        for (pubkey, account) in accounts.iter() {
            println!("Account: {:?}, Data: {:?}", pubkey, account.data);
        }

        Ok(())
    }

    pub async fn get_balance(&self, account_pubkey: &str) -> Result<(), Box<dyn Error>> {
        let pubkey = Pubkey::from_str(account_pubkey)?;
        let balance = self.client.get_balance(&pubkey)?;
        println!("Balance for account {}: {}", account_pubkey, balance);
        Ok(())
    }

    pub async fn get_transaction_count(&self) -> Result<(), Box<dyn Error>> {
        let transaction_count = self.client.get_transaction_count()?;
        println!("Transaction count: {}", transaction_count);
        Ok(())
    }

    pub async fn export_accounts_json(&self, program_id: &str, filename: &str) -> Result<(), Box<dyn Error>> {
        let pubkey = Pubkey::from_str(program_id)?;
        let accounts = self.client.get_program_accounts(&pubkey)?;

        let json_data = json!(accounts);
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(json_data.to_string().as_bytes())?;

        println!("Data exported to {}", filename);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let indexer = SolanaIndexer::new(&args.rpc_url);

    match &args.command {
        Command::FetchAccounts => {
            if let Some(program_id) = &args.program_id {
                if let Err(e) = indexer.fetch_program_accounts(program_id).await {
                    eprintln!("Error fetching accounts: {}", e);
                }
            } else {
                eprintln!("Program ID is required for fetching accounts.");
            }
        }
        Command::GetBalance { account_pubkey } => {
            if let Err(e) = indexer.get_balance(account_pubkey).await {
                eprintln!("Error fetching balance: {}", e);
            }
        }
        Command::GetTransactionCount => {
            if let Err(e) = indexer.get_transaction_count().await {
                eprintln!("Error fetching transaction count: {}", e);
            }
        }
        Command::ExportAccountsJson { filename } => {
            if let Some(program_id) = &args.program_id {
                if let Err(e) = indexer.export_accounts_json(program_id, filename).await {
                    eprintln!("Error exporting accounts to JSON: {}", e);
                }
            } else {
                eprintln!("Program ID is required for exporting accounts to JSON.");
            }
        }
    }
}
