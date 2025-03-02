use simple_logger::SimpleLogger;

mod ownership;
mod concurrency;

// by default, it's set to the "multi_thread" runtime && default worker threads == available CPU cores,
// but can be modified and explicitly set to #[tokio::main(flavor = "single_thread")]
// or if you want to constraint worker threads amount, then #[tokio::main(flavor = "multi_thread", worker_threads = X)]
#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let _ = concurrency::basics().await;
}