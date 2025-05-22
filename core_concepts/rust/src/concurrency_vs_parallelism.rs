const WORKERS: usize = 5;


async fn primitive_concurrency() -> () {
    let mut handlers: Vec<tokio::task::JoinHandle<()>> = Vec::with_capacity(WORKERS);

    for i in 0..WORKERS {
        let handle = tokio::task::spawn(async move {
            println!("CONCURRENCY: {i} || thread id: {:?}", std::thread::current().id());  // NOTE, im accessing thread::id , NOT a task::id 
        });

        handlers.push(handle);
    }

    for handle in handlers {
        let _ = handle.await;
    }    
}

fn primitive_parallelism() -> () {
    let mut handlers: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(WORKERS);

    for i in 0..WORKERS {
        let handle = std::thread::spawn(move || {
            println!("PARALLELISM: {i} || thread id: {:?}", std::thread::current().id());
        });

        handlers.push(handle);
    }

    for handle in handlers {
        let _ = handle.join().unwrap();
    }
}

pub fn try_example() -> std::io::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;  // default runtime is sufficient enough
    rt.block_on(primitive_concurrency());  // wanna wait and block the current thread until the completion => use block_on() instead of spawn()
    
    println!("\n----\n");

    primitive_parallelism();

    // the output is:

    // CONCURRENCY: 0 || thread id: ThreadId(12)
    // CONCURRENCY: 1 || thread id: ThreadId(12)
    // CONCURRENCY: 2 || thread id: ThreadId(12)
    // CONCURRENCY: 3 || thread id: ThreadId(12)
    // CONCURRENCY: 4 || thread id: ThreadId(12)

    // ----

    // PARALLELISM: 1 || thread id: ThreadId(15)
    // PARALLELISM: 0 || thread id: ThreadId(14)
    // PARALLELISM: 2 || thread id: ThreadId(16)
    // PARALLELISM: 3 || thread id: ThreadId(17)
    // PARALLELISM: 4 || thread id: ThreadId(18)


    // Concurrency output explanation:
    // Spawned tasks, managed by Tokio runtime are executed at the single thread in this case 
    //   (the result will differ from time to time, try rerunning this code),
    //   even though our runtime configuration uses multi-threaded environment (can be changed manually to only use a single thread),  
    //   it's possible to see many tasks sitting at the thread, let's say with an ID: 10, whereas some tasks will sit at the thread with an ID: 11
    // so it's okay to observe the following:
    // CONCURRENCY: 1 || thread id: ThreadId(14)
    // CONCURRENCY: 2 || thread id: ThreadId(13)    <---- ID: 13
    // CONCURRENCY: 3 || thread id: ThreadId(14)
    // CONCURRENCY: 4 || thread id: ThreadId(14)
    // CONCURRENCY: 0 || thread id: ThreadId(13)    <---- ID: 13


    // Parallelism output explanation:
    // And, as expected, primitive parallelism shows us that the each thread has it's own unique ID.
    //    BUT DON'T forget, even though the thread ID is unique, it may still share the single CPU core, this is possible thanks to "context switching"


    // ------------------------------------------------------------


    // More precisely:
    // Concurrency - provides the simultaneous execution by spawning multiple tasks, using single..multiple CPU cores (single/multiple OS threads). 
    //    Async runtime (e.g. tokio) automatically manages the spawned (cheap) tasks between available (expensive) OS threads. 

    // Parallelism - provides the simultaneous execution by spawning multiple OS threads, using X..max amount of available CPU cores (many OS threads).
    //   At any given instant, an OS thread runs on one core, but that core can host many threads over time via context switching. 
    //   This happens especially when there are more spawned OS threads than CPU cores!
    
    Ok(())
}