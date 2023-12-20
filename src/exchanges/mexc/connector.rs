use reqwest::header::HeaderMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::exchanges::signer::{sign, hex_encode};
use crate::utils::error::{HttpError, SignError};
use crate::utils::http_client::HttpClient;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signature {
    /// Signature.
    pub signature: String,

    /// Timestamp.
    pub timestamp: String,

    /// URI with timestamp and signature
    pub full_uri: String,
}

/// The APIKey definition of MEXC.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MexcConnector {
    /// MEXC_API_KEY
    pub api_key: String,
    /// MEXC_SECRET_KEY
    pub secret_key: String,

    pub data_source: String,
}

impl MexcConnector {
    pub fn new(api_key: String, secret_key: String) -> Self {
        let data_source = "MEXC".to_string();
        Self { api_key, secret_key, data_source }
    }

    pub fn sign(&self, uri: String) -> Result<Signature, SignError> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis().to_string();
        let total_parameters = uri.split("?").collect::<Vec<&str>>().get(1).unwrap().to_string();
        let raw_str = total_parameters.clone() + "&timestamp=" + timestamp.clone().as_str();
        let raw_sign = sign(raw_str.clone(), self.secret_key.clone())
            .expect(&*format!("Failed to create {} signature", self.data_source.clone()));;
        let encoded_sign = hex_encode(raw_sign);
        let full_uri = uri.clone() + "&timestamp=" + timestamp.clone().as_str() + "&signature=" + &*encoded_sign.as_str();

        Ok(Signature { signature: encoded_sign, timestamp, full_uri })
    }

    pub fn build_headers(&self) -> Result<HeaderMap, ()> {
        let mut headers = HeaderMap::new();
        headers.insert("X-MEXC-APIKEY", self.api_key.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        Ok(headers)
    }

    pub async fn http_client<T>(&self, url: String, uri: String, need_sign: bool) -> Result<T, HttpError>
        where
            T: serde::de::DeserializeOwned + Clone {
        let data_source = self.data_source.clone();
        let headers = self.build_headers().unwrap();

        let client = HttpClient::new(data_source.clone());

        let final_uri: String = if need_sign {
            self.sign(uri.clone()).unwrap().full_uri
        } else {
            uri
        };
        println!("{}", final_uri.clone());

        let response = client.send_request(url, final_uri, headers).await?;

        return if response.status().is_success() {
            let parsed_response = response
                .json::<T>()
                .await
                .expect(&*format!("[{}] Failed to deserialize response", data_source.clone()));
            Ok(parsed_response.clone())
        } else {
            eprintln!("[{data_source}] {:?}", response);
            Err(HttpError::ResponseError)
        };

        // match client.send_request(url, final_uri, headers).await {
        //     Ok(r) => {
        //         println!("{:?}", r);
        //         return if r.status().is_success() {
        //             let parsed_response = r
        //                 .json::<T>()
        //                 .await
        //                 .expect(&*format!("[{}] Failed to deserialize response", data_source.clone()));
        //             Ok(parsed_response.clone())
        //         } else {
        //             eprintln!("[{data_source}] {:?}", r);
        //             Err(HttpError::ResponseError)
        //         };
        //     }
        //     Err(e) => {
        //         eprintln!("[{data_source}] {:?}", e);
        //         Err(HttpError::ResponseError)
        //     }
        // }
    }
}

