use std::collections::HashMap;
use std::error::Error;
use http::HeaderMap;
use std::net::Ipv4Addr;

use crate::utils::http_client::HttpClient;

pub async fn monitor_ip() {
    let my_ip = my_ip().await.unwrap();
    println!(">>> My IP is {my_ip}\n")
}

async fn my_ip() -> Result<String, Box<dyn Error>> {
    let data_source = "my_ip".to_string();
    let client = HttpClient::new(data_source.clone());
    let mut headers = HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());

    let response = client.send_request(
        "https://api.ipify.org?".to_string(),
        "format=json".to_string(),
        headers
    ).await?;

    if response.status().is_success() {
        let json = response.json::<HashMap<String, String>>().await.unwrap();

        let ip: Ipv4Addr = json.get("ip").unwrap().parse().unwrap();
        Ok(ip.to_string())
    } else {
        panic!("[{data_source}] {:?}", response)
    }
}
