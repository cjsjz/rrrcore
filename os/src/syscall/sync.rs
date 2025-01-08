use crate::sync::{Semaphore,UPSafeCell};
use lazy_static::*;
use alloc::vec::Vec;


lazy_static! {
    pub static ref SEMAPHOR_VEC: UPSafeCell<Vec<Semaphore>> = unsafe { UPSafeCell::new(Vec::new()) };
}
/// Create a new semaphore with the given initial resource count
pub fn sys_semaphore_create(res_count: usize) -> isize {
    let sema = Semaphore::new(res_count);
    let mut sema_vec = SEMAPHOR_VEC.exclusive_access();
    sema_vec.push(sema);
    sema_vec.len() as isize
}

pub fn sys_semaphore_up(sem_id: usize) -> isize {
    let sema_vec = SEMAPHOR_VEC.exclusive_access();
    if sem_id >= sema_vec.len() {
        return -1;
    }
    sema_vec[sem_id-1].up();
    0
}
pub fn sys_semaphore_down(sem_id: usize) -> isize {
    let sema_vec = SEMAPHOR_VEC.exclusive_access();
    if sem_id >= sema_vec.len() {
        return -1;
    }
    sema_vec[sem_id-1].down();
    0
}
