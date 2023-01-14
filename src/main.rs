use clap::Parser;
use twig_rs::{contract::Contract, query, sub};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]

struct Cli {
    #[command(subcommand)]
    dispatcher: sub::Commands,
}

fn main() {
    let cli = Cli::parse();
    let rt = tokio::runtime::Runtime::new().unwrap();

    match &cli.dispatcher {
        sub::Commands::Find {
            rpc_url,
            contract_address,
        } => {
            println!(
                "rpc url provided {} , Contract Address {}",
                rpc_url, contract_address
            );

            let bytecode = rt
                .block_on(query::get_code(rpc_url, contract_address))
                .unwrap();

            let contract = Contract { bytecode };

            contract.get_dispatcher();
        }
    }
}
