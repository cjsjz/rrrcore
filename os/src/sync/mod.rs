//! Synchronization and interior mutability primitives
mod up;
mod semaphore;

pub use up::UPSafeCell;
pub use semaphore::Semaphore;