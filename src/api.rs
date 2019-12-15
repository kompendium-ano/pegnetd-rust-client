use super::*;
use serde::Serialize;

pub struct Pegnetd {
  pub client: reqwest::Client,
  pub node: &'static str
}

impl Pegnetd {
  pub fn new() -> Self {
    Self {
      client: reqwest::Client::new(),
      node: LOCAL_NODE
    }
  }

  pub fn open_node() -> Self {
    Self {
      client: reqwest::Client::new(),
      node: OPEN_NODE
    }
  }

  pub fn custom_node(node: &'static str) -> Self {
    Self {
      client: reqwest::Client::new(),
      node
    }
  }
}

/// Return the current heights synced by pegnetd and the factomd it is communicating with.
pub async fn sync_status(api: &Pegnetd) -> ApiResponse<SyncStatus> {
  let req = requests::ApiRequest::new("get-sync-status");
  request(api, req).await
}

/// Returns the given pegnet transaction if it exists. All txids are in the format [TxIndex]-[TxHash]. If the hash exists, but not the index, the result is Transaction Not Found. The response is still an array, as the underlying functionality is provided by the get-transactions call. You can see the documentation for that call for additional parameters. The main difference is that this call only accepts a txid as the search parameter. All calls should return at most 1 transaction in the array.
pub async fn transaction(api: &Pegnetd, txid: &str) -> ApiResponse<Transaction> {
  let mut req = requests::ApiRequest::new("get-transaction");
  req.params.insert("txid".to_string(), json!(txid));
  request(api, req).await
}

/// Get the total supply for each pegnet asset.
pub async fn issuance(api: &Pegnetd) -> ApiResponse<PegnetIssuance> {
  let req = requests::ApiRequest::new("get-pegnet-issuance");
  request(api, req).await
}

/// Get the pegnet asset balances for a given address. All balances are in their fixed point values, meaning a balance of 1e8 == 1 pAsset. In this example response, the address has 9401 PEG.
pub async fn balances(api: &Pegnetd, address: &str) -> ApiResponse<Assets> {
  let mut req = requests::ApiRequest::new("get-pegnet-balances");
  req.params.insert("address".to_string(), json!(address));
  request(api, req).await
}

/// Returns the pegnet conversion rates for a given block height.
pub async fn rates(api: &Pegnetd) -> ApiResponse<Assets> {
  let req = requests::ApiRequest::new("get-pegnet-rates");
  request(api, req).await
}

/// Returns the status of a transaction. The parameter entryhash can be either a winning OPR entry hash, a transaction chain entry hash, or an FCT Burn transaction id. An unknown hash will result in an -32803 error. A known hash response contains two fields:
/// 
/// height: this is the directory block height that the transaction was submitted
/// executed: this is the status indicator
/// -3 means the transaction was rejected due to being a pXXX -> pFCT transaction while one-way conversions are enabled for pFCT.
/// -2 means the transaction was rejected due to being invalid. This could be an expired timestamp, bad signature, or other validation rules.
/// -1 means the transaction was rejected due to insufficient balance
/// 0 means the transaction is currently pending and has not yet been processed
/// N (>= height) means the transaction was successfully applied at block height N.
pub async fn transaction_status(api: &Pegnetd, entryhash: &str) -> ApiResponse<TxStatus> {
  let mut req = requests::ApiRequest::new("get-transaction-status");
  req.params.insert("entryhash".to_string(), json!(entryhash));
  request(api, req).await
}

/// Input data for the transactions function
/// 
/// Returns a set of up to 50 transactions for the given parameters. The only requirement is setting either entryhash, address, or height. All other parameters are optional.
/// 
/// entryhash (hex-encoded string): can be either a winning OPR entry hash, a transaction chain entry hash, or an FCT Burn transaction id, e.g.: a33d4f334a2658c17d3f44158af75f1c32cc6b2f3de9ddc337064c93043d8db0. Mutually exclusive with address and height.
/// address (string): transactions involving the address (as sender or recipient), e.g.: FA3pPBWaVjZXhiFiUBdpvyjK84cvjWcZvZqoxeAEsH3rFCtEvEVt. Mutually exclusive with entryhash and height.
/// height (integer): transactions that were submitted at this height, e.g.: 213438
/// offset (integer): Results are paginated and the offset is used to specify the starting point. A paginated result will return the next value of offset as nextoffset. e.g.: 150
/// desc (bool): If set to true, the order of transactions is reversed, retrieving newest transactions first.
/// transfer (bool): If set to true, the list of transactions will include transfers
/// conversion (bool): If set to true, the list of transactions will include conversions
/// coinbase (bool): If set to true, the list of transactions will include miner payouts
/// burn (bool): If set to true, the list of transactions will include fct burns
/// By default, all four transaction types are included. If one type is set to true, then ONLY that type is returned. You can combine different types, for example set transfer and conversion to true in order retrieve both transfers and conversions.
/// 
/// The results come in the form of up to 50 Transactions with a specific type. Each transaction has a unique (hash, tx_index) tuple but the hash alone is not unique. The data returned is a multi-level json format, with the base being:
/// 
/// actions (list): This contains the list of Transactions. For more, see below
/// count (integer): The total number of records for this request, which may be more than 50
/// nextoffset (integer): If this number is greater than zero, it means that not all transactions fit into the next. You can repeat the API request with offset to the value of nextoffset in order to retrieve the next set of transactions.
/// The list of transactions is an object with the following potential keys:
/// 
/// hash (hex encoded bytes): For transfers, conversions, and coinbases, this is the relevant entry hash. For burns, this is the txid.
/// txid ([TxIndex]-[Hash]): All hashes potentially contain multiple transactions. Every transaction associated with hash also has a transaction index. By using the transaction index as a prefix to the hash, a specific transaction in the set can be identified.
/// height (integer): The height that the transaction was written to the chain.
/// timestamp (string): The timestamp of when the transaction was written to the chain.
/// executed (integer): Status indicator. <0 means rejected, 0 means pending, otherwise, it is the height the transaction was applied. All rejected codes can be found in the get-transaction-status: api description.
/// txindex (integer): A single entry can contain multiple transactions and this is the index of the transaction inside of an entry. For coinbases and burns, this is always 0.
/// txaction (integer): The type of Transaction:
/// 1: Transfer
/// 2: Conversion
/// 3: Coinbase
/// 4: Burn
/// fromaddress (string): The origin of the funds in the form of an FA... address. For Conversions, Coinbases, and Burns this is also the recipient.
/// fromasset (string): The pAsset that was sent. For Burns this is always "FCT", for Coinbases this is blank.
/// fromamount (integer): The amount was sent in Pegtoshis. For Coinbases, this is blank.
/// toasset (string): The destination pAsset. For Transfers, this is blank.
/// toamount (integer): The amount of pAsset the address was credited with. For pending Conversions, this is 0, otherwise it contains the actual amount. For Transfers, this is 0.
/// outputs (list): For transfers, this is a list that contains the set of outputs. There is at least one output in the list. For conversions into PEG (pXXX -> PEG), the outputs can also contain a refund amount in the original pXXX asset. Since PEG conversions have a limit, partial conversions exist. For all other types, this is blank.
/// The list of outputs contains objects with an address and an amount field. The amount units are always in fromasset units
/// # Example
/// 
/// ```rust
///   let api = Pegnetd::open_node();
///   let entryhash = Some("be802ffdb45f4a0a03d93ced1e3690b813d78921d94f3f642dca8ad3b33a506a".to_string());
///   let params = TxParams{
///     entryhash,
///     ..Default::default()
///   };
///   let response = transactions(&api, params).await;
///   dbg!(response);
pub async fn transactions(
  api: &Pegnetd, 
  params: TxParams
) -> ApiResponse<Transactions> 
{
  let mut req = requests::TxsApiRequest::new("get-transactions");
  req.params = json!(params);
  request(api, req).await
}

#[derive(Default, Clone, Debug, Serialize)] 
pub struct TxParams {
  pub entryhash: Option<String>,
  pub address: Option<String>,
  pub height: Option<usize>,
  pub offset: Option<usize>,
  pub transfer: Option<bool>,
  pub conversion: Option<bool>,
  pub coinbase: Option<bool>,
  pub burn: Option<bool>
}