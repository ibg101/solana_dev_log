use tokio::sync::{
    mpsc,
    watch,
    oneshot,
    broadcast,
};


pub async fn basics() -> () {
    // Channels.
    // They are a convenient synchronization primitives, that provide communication between different tasks. 
    // There are 4 types of channels, that Tokio provides:
    // 1. mpsc
    // 2. watch
    // 3. oneshot
    // 4. broadcast

    // Use mpsc, if you need multiple producers (of the value) and a single consumer.
    // So think of this - MANY to ONE with buffer(X).
    mpsc_example().await;

    // Use watch, if you need muttiple producers and multiple consumers, BUT consider that watch::channel() stores only the most recent value.
    // So think of this - MANY to MANY with buffer(1).
    // TIP: useful for state synchronization (creating signals).  
    watch_example().await;

    // Use oneshot, if you need single producer and single consumer.
    // So think of this - ONE to ONE with buffer(1).
    // TIP: useful for sending computation result between tasks.
    oneshot_example().await;

    // Use broadcast, if you need multiple producers and multiple consumers.
    // So think of this - MANY to MANY with buffer(X).
    // By the way, there is no dedicated ONE to MANY channel, so if you want to have single producer and multiple consumers -> use broadcast
    // TIP: may be used in chat systems.
    broadcast_example().await;  
}

async fn mpsc_example() -> () {
    const BUFFER: usize = 10;
    let (tx, mut rx) = mpsc::channel::<usize>(BUFFER);

    // Spawning separated task, so we can SEND & RECEIVE values AT A TIME
    tokio::task::spawn(async move {
        for i in 0..BUFFER {
            if let Err(e) = tx.send(i).await {
                log::error!("{}", e);
                return;
            }
        }
    });

    // Receiver, that consumes sent values. 
    // In this impl it's not isolated in the separated task, and since it's a loop -> will prevent other tasks from executing, before completes 
    while let Some(i) = rx.recv().await {
        log::info!("Received value: {}", i);
    }
}

async fn watch_example() -> () {
    let init_state: bool = false;
    let (tx, mut rx) = watch::channel(init_state);

    tokio::task::spawn(async move {
        loop {
            // if value is unchanged and unseen -> sleeps & doesn't loop    
            if rx.changed().await.is_ok() {
                let state: bool = *rx.borrow_and_update();
                log::info!("State changed: {}", state);
                break;  // if you need to monitor continuously, omit break
            }
        }
    });

    // simulating delay
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    let _ = tx.send(!init_state);
}

async fn oneshot_example() -> () {
    let (tx, rx) = oneshot::channel::<u64>();

    // consumer task, that waits for the value
    tokio::task::spawn(async move {
        match rx.await {
            Ok(v) => log::info!("Received value: {}", v),
            Err(e) => log::error!("{}", e)
        }
    });

    // simulating delay
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    // sending some value
    if let Err(_) = tx.send(1u64) {
        log::error!("Receiver dropped!");
    }
}

async fn broadcast_example() -> () {
    let (tx1, mut rx1) = broadcast::channel::<&[u8]>(1);
    let mut rx2: broadcast::Receiver<&[u8]> = tx1.subscribe();  // creating 2nd receiver by subscribing to the same channel 

    let _ = tx1.send("101".as_bytes());

    // Receiver 1 receives the same value as Receiver 2 does.
    // Single producer - Multiple Consumers
    if let Ok(v) = rx1.recv().await {
        let msg: &str = unsafe { std::str::from_utf8_unchecked(v) };
        log::info!("Receiver 1: {msg}");
    }

    if let Ok(v) = rx2.recv().await {
        let msg: &str = unsafe { std::str::from_utf8_unchecked(v) };
        log::info!("Receiver 2: {msg}");
    }
}