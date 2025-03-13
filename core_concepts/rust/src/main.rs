use simple_logger::SimpleLogger;

mod std_lib;
mod tokio_lib;
mod rpc;

// by default, it's set to the "multi_thread" runtime && default worker threads == available CPU cores,
// but can be modified and explicitly set to #[tokio::main(flavor = "single_thread")]
// or if you want to constraint worker threads amount, then #[tokio::main(flavor = "multi_thread", worker_threads = X)]
#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    // let _ = tokio_lib::concurrency::basics().await;
    // tokio_lib::channels::basics().await;

    let sig: &str = "yn5n7ke3b59mVaUTJVKb6gA9C5xr2jVeqadKk1vUDMjAX6LrCYi365D8qrRsDLC3TwGpCPvnt24w37fqL3mLevP";
    rpc::communication::get_transaction(
        "https://api.mainnet-beta.solana.com",  // your_rpc_provider_http_url 
        sig, 
        rpc::communication::CommitmentLevel::Confirmed
    ).await.unwrap();

    let account_id: &str = "3AbG3ZA19fJKjTSTMTCz7j2bodPagXog4PwTBi8H7UA4";
    rpc::communication::account_subscribe(
        "wss://api.mainnet-beta.solana.com",  // your rpc_provider_ws_url 
        account_id, 
        rpc::communication::CommitmentLevel::Confirmed
    ).await.unwrap();
}