use clap::{Subcommand};

#[derive(Subcommand)]
pub enum DispatcherCommands {
    /// Dispatcher commands
    Find {
        /// Url to connect with an rpc
        #[clap(short = 'r', long = "rpc-url")]
        rpc_url: String,

        /// Contract address to query
        #[clap(short = 'c', long = "cont-addr")]
        contract_address: String,
    },
}
