use std::sync::atomic::{AtomicI32, AtomicBool, Ordering};

pub static ALLPARTS: bool = cfg!(feature="allparts");

pub static N_STRIPS: AtomicI32  = AtomicI32::new(8);
pub static BREZIER:  AtomicBool = AtomicBool::new(false);
pub static BINARY:   AtomicBool = AtomicBool::new(false);

pub trait EasyAtomic<T> {
    type T;
    fn get(&self) -> T;
    fn set(&self, value: T);
}

impl EasyAtomic<i32> for AtomicI32 {
    type T = i32;
    fn get(&self) -> i32 { self.load(Ordering::Relaxed) }
    fn set(&self, value: i32) { self.store(value, Ordering::Relaxed) }
}

impl EasyAtomic<bool> for AtomicBool {
    type T = bool;
    fn get(&self) -> bool { self.load(Ordering::Relaxed) }
    fn set(&self, value: bool) { self.store(value, Ordering::Relaxed) }
}