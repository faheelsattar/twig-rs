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
            rpc_url,
            contract_address,
        } => {
            println!(
                "Rpc url provided {} , Contract Address {}",
                rpc_url, contract_address
            )
        }
    }
}
