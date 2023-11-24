use std::env;

use ethers::providers::{Http, Provider};

pub fn build_ethers_provider(chain_id: u32) -> Provider<Http> {
    let node_env_name = format!("NODE_URL_{chain_id}");
    let node_url = env::var(node_env_name)
        .expect(&*format!("Node provider url of chain id {chain_id} not found"));
    return Provider::try_from(node_url).expect("Failed to build a ethers provider");
}
