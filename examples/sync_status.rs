use pegnetd::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  let api = Pegnetd::open_node();
  let response = sync_status(&api).await;
  assert!(response.result.syncheight > 0);
  Ok(())
}