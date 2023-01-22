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
        contract_address: Box<str>,
    },

    /// Call subcommand queries the view only function of the smart contract
    Call {
        /// Url to connect with an rpc
        #[clap(short = 'r', long = "rpc-url")]
        rpc_url: Box<str>,

        /// Contract address to query
        #[clap(short = 'c', long = "contract-address")]
        contract_address: Box<str>,

        /// Function to call
        #[clap(short = 'f', long = "function-signature")]
        function_signature: Box<str>,

        /// Function args
        #[clap(short = 'a', long = "function-args", num_args = 1.., value_delimiter = ' ')]
        function_args: Vec<Box<str>>,
    },
}
