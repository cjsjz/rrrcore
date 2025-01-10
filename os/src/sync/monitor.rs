use crate::sync::UPSafeCell;
use crate::syscall::{sys_semaphore_create, sys_semaphore_down, sys_semaphore_up};
/// A monitor is a synchronization primitive that allows threads to wait for each other
pub struct Monitor {
    /// The inner monitor data structure
    inner: UPSafeCell<MonitorInner>,
}
/// A monitor is a synchronization primitive that allows threads to wait for each other
pub struct MonitorInner {
    /// The semaphore used to protect the monitor
    pub mutex : usize,
    /// The semaphore used to signal waiting emergency process
    pub next : usize,
    /// The number of waiting emergency process
    pub next_count : usize,
    /// The number of waiting threads
    pub x_count : [isize; 10],
}
impl Monitor {
    /// Create a new monitor
    pub fn new() -> Self {
        Self {
            inner: unsafe { UPSafeCell::new(MonitorInner {
                mutex : sys_semaphore_create(1) as usize,
                next : sys_semaphore_create(0) as usize,
                next_count : 0,
                x_count : [0; 10],
            }) },
        }
    }
    ///Enter the monitor
    pub fn enter(&self) {
        // Acquire the mutex semaphore
        let inner = self.inner.exclusive_access();
        sys_semaphore_down(inner.mutex);
    }
    ///Exit the monitor
    pub fn leave(&self) {
        // Release the mutex semaphore
        let inner = self.inner.exclusive_access();
        if inner.next_count > 0 {
            sys_semaphore_up(inner.next);
        } else {
            sys_semaphore_up(inner.mutex);
        }
    }
    ///Wait for the monitor
    pub fn wait(&self, x_sema_id: usize) {
        // Acquire the x_semaphore semaphore
        let mut inner = self.inner.exclusive_access();
        inner.x_count[x_sema_id] += 1;
        if inner.next_count > 0 {
            sys_semaphore_up(inner.next);
        } else {
            sys_semaphore_up(inner.mutex);
            }
        sys_semaphore_down(x_sema_id);
        inner.x_count[x_sema_id] -= 1;
    }
    ///Signal the monitor
    pub fn signal(&self, x_sema_id: usize) {
        let mut inner = self.inner.exclusive_access();
        if inner.x_count[x_sema_id] > 0 {
            inner.next_count += 1;
            sys_semaphore_up(x_sema_id);
            sys_semaphore_down(inner.next);
            inner.next_count -= 1;
        }
    }
    ///Set value of the condition variable
    pub fn set_value(&self, x_sema_id: usize, value: isize) {
        let mut inner = self.inner.exclusive_access();
        inner.x_count[x_sema_id] = value;
    }
}