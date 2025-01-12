use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub(crate) struct WatchLock {
    locked: Arc<AtomicBool>,
}

impl WatchLock {
    pub(crate) fn new() -> Self {
        Self {
            locked: Arc::new(AtomicBool::new(false)),
        }
    }
    
    pub(crate) fn is_locked(&self) -> bool {
        self.locked.load(std::sync::atomic::Ordering::Relaxed)
    }
    
    pub(crate) fn lock(&self) {
        self.locked.store(true, std::sync::atomic::Ordering::Relaxed);
    }
 
    pub(crate) fn unlock(&self) {
        self.locked.store(false, std::sync::atomic::Ordering::Relaxed);
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
    
    #[test]
    fn lock_can_be_set() {
        let lock = WatchLock::new();
        
        lock.lock();
        
        assert!(lock.is_locked());
    }
    
    #[test]
    fn lock_can_be_unlocked() {
        let lock = WatchLock::new();

        lock.lock();
        lock.unlock();

        assert!(!lock.is_locked());
    }
}