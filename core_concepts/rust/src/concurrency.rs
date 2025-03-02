use std::{
    fs::{self, ReadDir}, 
    io::{self, Read}, 
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering} 
    }
};
use tokio::sync::{
    Mutex, 
    MutexGuard
};


pub async fn basics() -> io::Result<()> {
    // By default, Tokio runs in a multi-threaded environment. 
    // Blocking operations in a single-threaded runtime will prevent other async tasks from running, 
    // but in the default multi-threaded runtime, they only block the current worker thread.
    
    // Simulating IO operation in a single task.
    log::info!("Task is paused!");
    // By default, when the .await breakpoint is reached - control flow is yielded back to the Tokio Runtime,
    // in order to allow other tasks run concurrently/in parallel.
    // However, in a single-task scenario (like this example), there are no other tasks to execute,
    // so the runtime waits for this sleep to resolve before proceeding.
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    log::info!("Task is resumed!");

    // Q: How to run the code without waiting for completion of the previous async operation?
    // A: There are 2 ways to work around this:
    //   1. using tokio::join!() macro -> concurrency
    //   2. using separated tasks -> parallelism

    // 1. tokio::join!()
    // This macro allows to run different operations concurrently, but in a single thread.
    // Even thought it allows different operations run concurrently, only one task can run a time in this scenario. 
    // Example, Execution Flow: 
    // id_0 - Operation 2
    // id_1 - Operation 1
    tokio::join!(
        async {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            log::info!("Operation 1 is finished!");
        },
        async {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            log::info!("Operation 2 is finished!");
        }
    );

    // 2. Separated tasks with a tokio::task::spawn()
    // This allows to spawn different tasks, that are managed by the tokio runtime, 
    // which automatically distributes CPU cores (threads) between task pool.
    // => This leads to **true parallelism**, where all tasks can run at a time.
    
    // !!! In this Example i'm using unstable_feature (tokio::task::Builder), requires setting .cargo/config.toml + tracing flag in Cargo.toml.
    // You can use tokio::task::spawn() instead. 

    const WORKERS: usize = 10;
    let thread_safe_vec: Arc<Mutex<Vec<u8>>> = Arc::default();
    let mut workers: Vec<tokio::task::JoinHandle<()>> = Vec::with_capacity(WORKERS);

    for worker_id in 0..WORKERS as u8 {
        // incrementing atomic strong reference counter
        let vec_light_clone: Arc<Mutex<Vec<u8>>> = Arc::clone(&thread_safe_vec);

        let worker_name: String = format!("Worker: {}", worker_id);

        let handle: tokio::task::JoinHandle<()> = tokio::task::Builder::new()
            .name(&worker_name)
            .spawn(async move {
                log::info!("Worker {} start!", worker_id);
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;  // simulating IO delay
                let mut vec_guard: MutexGuard<Vec<u8>> = vec_light_clone.lock().await;
                vec_guard.push(worker_id);
                log::info!("Worker {} finish!", worker_id); 
            })?;
        workers.push(handle);
    }

    futures_util::future::join_all(workers).await;  // waiting, until all workers complete 

    assert!(thread_safe_vec.lock().await.len() == WORKERS);

    // 2. Separated tasks with a tokio::task::spawn_blocking()
    // This method is used in following scenarios:
    // - compute-heavy (File operations, CPU intensive work) 
    // - sync (where you need to have a sync block of code in async environment).
    // let dir_iter: fs::ReadDir = fs::read_dir("./example_files")?;
    let dir_iter: ReadDir = tokio::task::spawn_blocking(|| { fs::read_dir("./example_files") }).await??;
    let total_bytes: Arc<AtomicUsize> = Arc::default(); 
    let mut workers: Vec<tokio::task::JoinHandle<io::Result<()>>> = Vec::new();
    
    for unchecked_dir_entry in dir_iter {
        if let Ok(dir_entry) = unchecked_dir_entry {
            let total_bytes_clone: Arc<AtomicUsize> = Arc::clone(&total_bytes);
            let handle: tokio::task::JoinHandle<io::Result<()>> = tokio::task::spawn_blocking(move || {
                let mut file: fs::File = fs::File::open(dir_entry.path())?;
                let mut buffer: String = String::new();
                file.read_to_string(&mut buffer)?;  // Blocking IO
                total_bytes_clone.fetch_add(buffer.len(), Ordering::Release);
                Ok(()) as io::Result<()>
            });
            workers.push(handle);            
        } 
    }

    futures_util::future::join_all(workers).await;
    log::info!("Total bytes in all opened files: {}", total_bytes.load(Ordering::Acquire));

    Ok(())
}