use std::str::FromStr;

use ethers::{
    providers::{Http, Middleware, Provider, ProviderError},
    types::{Address, BlockId, BlockNumber, Bytes},
};

pub async fn get_code(rpc_url: &str, contract_address: &str) -> Result<Bytes, ProviderError> {
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();

    let bytecode = match provider
        .get_code(
            Address::from_str(contract_address).unwrap(),
            Some(BlockId::Number(BlockNumber::Latest)),
        )
        .await
    {
        Ok(data) => {
            println!("Bytecode: {}", data);
            data
        }
        Err(err) => return Err(err),
    };

    Ok(bytecode)
}
