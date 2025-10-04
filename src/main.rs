use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

async fn philosophers(n: usize) {
    let forks: Vec<_> = (0..n).map(|_| Arc::new(Mutex::new(()))).collect();
    let mut handles = vec![];

    for i in 0..n {
        let left = Arc::clone(&forks[i]);
        let right = Arc::clone(&forks[(i + 1) % n]);
        
        handles.push(tokio::spawn(async move {
            loop {
                sleep(Duration::from_millis(1)).await;
                let _left = left.lock().await;
                let _right = right.lock().await;
                sleep(Duration::from_millis(1)).await;
            }
        }));
    }

    for h in handles {
        h.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    philosophers(5).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_deadlock(n in 2usize..8) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let handle = rt.spawn(async move { philosophers(n).await });
            std::thread::sleep(std::time::Duration::from_secs(1));
            prop_assert!(!handle.is_finished(), "Should deadlock!");
        }
    }
}
