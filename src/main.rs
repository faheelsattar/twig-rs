use clap::Parser;
use twig_rs::{contract::Contract, query, sub};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]

struct Cli {
    #[command(subcommand)]
    sub: sub::Commands,
}

fn main() {
    let cli = Cli::parse();
    let rt = tokio::runtime::Runtime::new().unwrap();

    match &cli.sub {
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
            let pattern: Vec<u8> = vec![0x80, 0x63, 0x14, 0x61, 0x57];
            contract.extract_dispatcher(pattern);
        }
        sub::Commands::Call {
            rpc_url,
            contract_address,
            function_signature,
            function_args,
        } => {
            for args in function_args.iter() {
                println!("args  {}", args)
            }
        }
    }
}
