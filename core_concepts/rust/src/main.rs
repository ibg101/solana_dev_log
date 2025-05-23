// requires nightly! if it's not an option, remove this along with benches
#![feature(test)]  
extern crate test;

use simple_logger::SimpleLogger;

mod std_lib;
mod tokio_lib;
mod rpc;
mod concurrency_vs_parallelism;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::UnsafeCell, sync::Arc};
    use test::{Bencher, black_box};

    #[test]  // this example sets it's own runtime, no need in #[tokio::test]
    fn concurrency_vs_parallelism() -> () {
        concurrency_vs_parallelism::try_example().unwrap();
    }

    const ITERS: usize = 1_000_000;

    #[bench]
    fn share_same_cache_line(b: &mut Bencher) -> () {
        // False sharing:
        // Occurs when two or more CPU cores modify variables that live on the SAME cache line (typically 64 bytes).
        // Even if the cores are working with competely different variables, modifying one invalidates the entire cache line on other cores,
        //   leading to the need of sharing the updated state.
        // 
        // Values appear in the same cache line if they are in Vec for example (because they are laid out back-to-back)
        //   and if it has an offset of N..64 
        // 
        // There are 4 states of cache line - MESI:
        // MESI - Modified, Exclusive, Shared, Invalid.

        // Example of False Sharing:
        //   vec![1_u64, 2_u64] - vec[0] and vec[1] share the same cache line.
        //   if vec[0] is modified, the whole cache line (in this case 8 elements of size u64) becomes invalid,
        //   leading to downtime until the cache line shares the updated state, even though they are completely different values & memory locations.

        // In order to prevent this #[repr(align(64))] in pair with internal padding is used.
        // But let's illustrate the problem first.


        // This example illustrates False Sharing due to:
        // - cache line is usually 64 bytes
        // - a & b => laid out back-to-back in memory, meaning they are contiguous
        // - because they are contiguous they will share the same cache line, since each field has the size of 8 bytes (8 + 8 can be placed in 64)
        #[derive(Default)]
        struct Foo {
            a: UnsafeCell<u64>,
            b: UnsafeCell<u64>,  // `b`'s offset here is 0x8..0x10 => 8..16 bytes => shares the same cache line
        }

        // UnsafeCell doesn't implement Sync due to safety guarantees, but in our scenario it's absolutely safe to impl Sync,
        //   because threads do not access the same memory location
        unsafe impl Sync for Foo {}

        b.iter(|| {
            let f1 = Arc::new(Foo::default());
            let f2 = Arc::clone(&f1);

            // accesses fiald.a
            let handle1 = std::thread::spawn(move || {
                for i in 0..ITERS {
                    black_box(
                    unsafe { *f1.a.get() += i as u64; }
                    )
                }
            });

            // accesses field.b
            let handle2 = std::thread::spawn(move || {
                for i in 0..ITERS {
                    black_box(
                    unsafe { *f2.b.get() += i as u64; }
                    )
                }
            });

            handle1.join().unwrap();
            handle2.join().unwrap();
        });
    }

    #[bench]
    fn prevent_false_sharing(b: &mut Bencher) -> () {
        #[derive(Default)]
        #[repr(align(64))]
        struct Foo {
            a: UnsafeCell<u64>,
            _pad: [u64; 7],      //  if we omit this explicit internal padding, `field.b` will share the cache line
            b: UnsafeCell<u64>,  // `b`'s offset is 0x40..0x48 => 64..72 bytes => new cache line
            _pad2: [u64; 7],     // this can be omitted, not necessary, but leave for explicitness
        }

        // UnsafeCell doesn't implement Sync due to safety guarantees, but in our scenario it's absolutely safe to impl Sync,
        //   because threads do not access the same memory location
        unsafe impl Sync for Foo {}

        b.iter(|| {
            let f1 = Arc::new(Foo::default());
            let f2 = Arc::clone(&f1);

            // accesses fiald.a
            let handle1 = std::thread::spawn(move || {
                for i in 0..ITERS {
                    black_box(
                    unsafe { *f1.a.get() += i as u64; }
                    )
                }
            });

            // accesses field.b
            let handle2 = std::thread::spawn(move || {
                for i in 0..ITERS {
                    black_box(
                    unsafe { *f2.b.get() += i as u64; }
                    )
                }
            });

            handle1.join().unwrap();
            handle2.join().unwrap();
        });
    }
}