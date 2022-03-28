use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::task::JoinHandle;
use std::time::{Duration, Instant};
use crossbeam::channel::tick;
use tokio::spawn;
use crate::Task;


pub async fn start_agent() {
    let n = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    let ticker = tick(Duration::from_millis(1000));

    let task = Task::new(10);

    loop {
        let _ = ticker.recv().unwrap();
        let mut x = n.clone();
        let t = task.clone();

        tokio::spawn(async move {
            println!("task parallel: {:?}", x.load(Ordering::Relaxed) < task.parallel);
            if x.load(Ordering::Relaxed) < task.parallel {
                x.fetch_add(1, Ordering::SeqCst);
                t.run().await;
                x.fetch_sub(1, Ordering::SeqCst);
            }
        });
    }
}