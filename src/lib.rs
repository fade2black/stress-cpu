//! Utility that simulates a high cpu load.
//! It creates `N` OS native threads each of which spins in a tight loop calculating the sqrt() of a random number.  
use rand::{thread_rng, Rng};
use std::{thread, time};

use tokio::sync::oneshot::{self, error::TryRecvError, Receiver};

fn compute(mut rx: Receiver<()>) {
    let mut rng = thread_rng();
    let num = rng.gen_range(1975..2005) as f64;

    while let Err(TryRecvError::Empty) = rx.try_recv() {
        let _ = f64::sqrt(num);
    }
}

/// Spawn `cpus` workers (native OS threads) spinning on `sqrt()` and timeouts after `timeout` seconds.
pub fn stress(cpus: usize, timeout: u64) {
    let mut handles = vec![];
    let mut channels = vec![];

    for _ in 0..cpus {
        let (tx, rx) = oneshot::channel();
        channels.push(tx);
        handles.push(std::thread::spawn(|| compute(rx)));
    }

    thread::sleep(time::Duration::from_secs(timeout));

    for tx in channels {
        tx.send(()).unwrap();
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
