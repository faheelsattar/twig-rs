use clap::Parser;
use twig_rs::dispatcher::DispatcherCommands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]

struct Cli {
    #[command(subcommand)]
    dispatcher: DispatcherCommands,
}

fn main() {
    let cli = Cli::parse();

    match &cli.dispatcher {
        DispatcherCommands::Find {
            etherscan_api,
            contract_address,
        } => {
            println!(
                "Etherscan api provided {} , Contract Address {}",
                etherscan_api, contract_address
            )
        }
    }
}
