use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Find subcommand lists all the external/public functions
    /// of the provided contract
    Find {
        /// Url to connect with an rpc
        #[clap(short = 'r', long = "rpc-url")]
        rpc_url: Box<str>,

        /// Contract address to query
        #[clap(short = 'c', long = "contract-address")]
        contract_address: Box<str >,
    },
}
