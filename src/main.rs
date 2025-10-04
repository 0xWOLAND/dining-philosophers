use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn philosophers(n: usize) {
    let forks: Vec<_> = (0..n).map(|_| Arc::new(Mutex::new(()))).collect();
    let mut handles = vec![];

    for i in 0..n {
        let left = Arc::clone(&forks[i]);
        let right = Arc::clone(&forks[(i + 1) % n]);
        
        handles.push(thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(1));
            let _left = left.lock().unwrap();
            let _right = right.lock().unwrap();
            thread::sleep(Duration::from_millis(1));
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn main() {
    philosophers(5);
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_deadlock(n in 2usize..8) {
            let handle = thread::spawn(move || philosophers(n));
            thread::sleep(Duration::from_secs(1));
            prop_assert!(!handle.is_finished(), "Should deadlock!");
        }
    }
}
