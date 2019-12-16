//! Contains the generic api response struct along with specific
//! structs for each indiviual endpoint

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
  pub height: usize,
  pub executed: usize,
}

/// Output strcut for get-sync-status call
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SyncStatus {
  pub syncheight: usize,
  pub factomheight: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Transaction {
  pub actions: Vec<Action>,
  pub count: usize,
  pub nextoffset: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Action {
  pub hash: String,
  pub txid: String,
  pub height: usize,
  pub timestamp: String,
  pub executed: usize,
  pub txindex: usize,
  pub txaction: usize,
  pub fromaddress: String,
  pub fromasset: String,
  pub fromamount: usize,
  pub toasset: String,
  pub toamount: usize,
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
  pub peg: usize,
  #[serde(rename = "pADA")]
  pub p_ada: usize,
  #[serde(rename = "pBNB")]
  pub p_bnb: usize,
  #[serde(rename = "pBRL")]
  pub p_brl: usize,
  #[serde(rename = "pCAD")]
  pub p_cad: usize,
  #[serde(rename = "pCHF")]
  pub p_chf: usize,
  #[serde(rename = "pCNY")]
  pub p_cny: usize,
  #[serde(rename = "pDASH")]
  pub p_dash: usize,
  #[serde(rename = "pDCR")]
  pub p_dcr: usize,
  #[serde(rename = "pETH")]
  pub p_eth: usize,
  #[serde(rename = "pEUR")]
  pub p_eur: usize,
  #[serde(rename = "pFCT")]
  pub p_fct: usize,
  #[serde(rename = "pGBP")]
  pub p_gbp: usize,
  #[serde(rename = "pHKD")]
  pub p_hkd: usize,
  #[serde(rename = "pINR")]
  pub p_inr: usize,
  #[serde(rename = "pJPY")]
  pub p_jpy: usize,
  #[serde(rename = "pKRW")]
  pub p_krw: usize,
  #[serde(rename = "pLTC")]
  pub p_ltc: usize,
  #[serde(rename = "pMXN")]
  pub p_mxn: usize,
  #[serde(rename = "pPHP")]
  pub p_php: usize,
  #[serde(rename = "pRVN")]
  pub p_rvn: usize,
  #[serde(rename = "pSGD")]
  pub p_sgd: usize,
  #[serde(rename = "pUSD")]
  pub p_usd: usize,
  #[serde(rename = "pXAG")]
  pub p_xag: usize,
  #[serde(rename = "pXAU")]
  pub p_xau: usize,
  #[serde(rename = "pXBC")]
  pub p_xbc: usize,
  #[serde(rename = "pXBT")]
  pub p_xbt: usize,
  #[serde(rename = "pXLM")]
  pub p_xlm: usize,
  #[serde(rename = "pXMR")]
  pub p_xmr: usize,
  #[serde(rename = "pZEC")]
  pub p_zec: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Transactions {
  pub actions: Vec<TxsAction>,
  pub count: usize,
  pub nextoffset: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TxsAction {
  pub hash: String,
  pub height: usize,
  pub timestamp: String,
  pub executed: isize,
  pub txindex: usize,
  pub txaction: usize,
  pub fromaddress: String,
  pub fromasset: String,
  pub fromamount: usize,
  pub toasset: Option<String>,
  pub toamount: Option<usize>,
  #[serde(default)]
  pub outputs: Vec<Output>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Output {
  pub address: String,
  pub amount: usize,
}