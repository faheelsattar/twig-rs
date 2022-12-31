use clap::{Subcommand};

#[derive(Subcommand)]
pub enum DispatcherCommands {
    /// Dispatcher commands
    Find {
        /// Url to connect with an rpc
        #[clap(short = 'e', long = "etherscan-url")]
        etherscan_api: String,

        /// Contract address to query
        #[clap(short = 'c', long = "cont-addr")]
        contract_address: String,
    },
}
