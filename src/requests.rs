//! General request handling and the ApiRequest struct

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde::de::DeserializeOwned;
use super::*;

/// Generic request type for api calls
pub async fn request<T, R>(api: &Pegnetd, req: R) -> ApiResponse<T>
    where T: DeserializeOwned + Default,
          R: Serialize
{
  let response = api.client
                      .post(api.node)
                      .json(&req)
                      .send()
                      .await
                      .unwrap()
                      .json()
                      .await
                      .unwrap();
  response
}

/// JSON-RPC serialisation struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
  pub jsonrpc: &'static str,
  pub id: usize,
  pub method: String,
  pub params: HashMap<String, Value>
}

impl ApiRequest{
  /// Creates a new ApiRequest 
  pub fn new(method: &str) -> ApiRequest{
    ApiRequest {
      jsonrpc: JSONRPC,
      id: ID,
      method: method.to_string(),
      params: HashMap::new()
    }
  }
}

/// For use with the Transctions API only
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxsApiRequest {
  pub jsonrpc: &'static str,
  pub id: usize,
  pub method: String,
  pub params: Value
}

impl TxsApiRequest{
  /// Creates a new TxsApiRequest using json values rather than a hashmap
  pub fn new(method: &str) -> Self{
    Self {
      jsonrpc: JSONRPC,
      id: ID,
      method: method.to_string(),
      params: json!(null)
    }
  }
}