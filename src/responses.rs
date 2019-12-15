use serde::Deserialize;
 
/// JSON responses are deserialized into this struct
#[derive(Default, Clone, Deserialize, Debug, PartialEq)]
pub struct ApiResponse<T>
  where T:  Default
{
  pub jsonrpc: String,
  pub id: usize,
  #[serde(default)]
  pub result: T,
  #[serde(default)]
  pub error: Error
}

// Generic Factom API Error struct
#[derive(Deserialize, Clone, PartialEq, Default, Debug)]
pub struct Error {
  pub code: i16,
  pub message: String
}

/// get-transaction-status function
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TxStatus {
  pub height: i64,
  pub executed: i64,
}

/// Output strcut for get-sync-status call
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SyncStatus {
  pub syncheight: i64,
  pub factomheight: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Transaction {
  pub actions: Vec<Action>,
  pub count: i64,
  pub nextoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Action {
  pub hash: String,
  pub txid: String,
  pub height: i64,
  pub timestamp: String,
  pub executed: i64,
  pub txindex: i64,
  pub txaction: i64,
  pub fromaddress: String,
  pub fromasset: String,
  pub fromamount: i64,
  pub toasset: String,
  pub toamount: i64,
}

/// get-pegnet-issuance function
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PegnetIssuance {
  pub syncstatus: SyncStatus,
  pub issuance: Assets,
}

// get-balances and get-pegnet-rates functions
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Assets {
  #[serde(rename = "PEG")]
  pub peg: i64,
  #[serde(rename = "pADA")]
  pub p_ada: i64,
  #[serde(rename = "pBNB")]
  pub p_bnb: i64,
  #[serde(rename = "pBRL")]
  pub p_brl: i64,
  #[serde(rename = "pCAD")]
  pub p_cad: i64,
  #[serde(rename = "pCHF")]
  pub p_chf: i64,
  #[serde(rename = "pCNY")]
  pub p_cny: i64,
  #[serde(rename = "pDASH")]
  pub p_dash: i64,
  #[serde(rename = "pDCR")]
  pub p_dcr: i64,
  #[serde(rename = "pETH")]
  pub p_eth: i64,
  #[serde(rename = "pEUR")]
  pub p_eur: i64,
  #[serde(rename = "pFCT")]
  pub p_fct: i64,
  #[serde(rename = "pGBP")]
  pub p_gbp: i64,
  #[serde(rename = "pHKD")]
  pub p_hkd: i64,
  #[serde(rename = "pINR")]
  pub p_inr: i64,
  #[serde(rename = "pJPY")]
  pub p_jpy: i64,
  #[serde(rename = "pKRW")]
  pub p_krw: i64,
  #[serde(rename = "pLTC")]
  pub p_ltc: i64,
  #[serde(rename = "pMXN")]
  pub p_mxn: i64,
  #[serde(rename = "pPHP")]
  pub p_php: i64,
  #[serde(rename = "pRVN")]
  pub p_rvn: i64,
  #[serde(rename = "pSGD")]
  pub p_sgd: i64,
  #[serde(rename = "pUSD")]
  pub p_usd: i64,
  #[serde(rename = "pXAG")]
  pub p_xag: i64,
  #[serde(rename = "pXAU")]
  pub p_xau: i64,
  #[serde(rename = "pXBC")]
  pub p_xbc: i64,
  #[serde(rename = "pXBT")]
  pub p_xbt: i64,
  #[serde(rename = "pXLM")]
  pub p_xlm: i64,
  #[serde(rename = "pXMR")]
  pub p_xmr: i64,
  #[serde(rename = "pZEC")]
  pub p_zec: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Transactions {
  pub actions: Vec<TxsAction>,
  pub count: i64,
  pub nextoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TxsAction {
  pub hash: String,
  pub height: i64,
  pub timestamp: String,
  pub executed: i64,
  pub txindex: i64,
  pub txaction: i64,
  pub fromaddress: String,
  pub fromasset: String,
  pub fromamount: i64,
  pub toasset: Option<String>,
  pub toamount: Option<i64>,
  #[serde(default)]
  pub outputs: Vec<Output>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Output {
  pub address: String,
  pub amount: i64,
}