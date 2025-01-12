use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub struct WatchLock {
    locked: Arc<AtomicBool>,
}

impl WatchLock {
    pub(crate) fn new() -> Self {
        Self {
            locked: Arc::new(AtomicBool::new(false)),
        }
    }
    
    pub fn is_locked(&self) -> bool {
        self.locked.load(std::sync::atomic::Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn new_lock_is_unlocked() {
        let lock = WatchLock::new();
        
        assert!(!lock.is_locked());
    }
}