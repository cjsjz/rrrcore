use crate::sync::{Semaphore,UPSafeCell};
use crate::timer::{add_timer, get_time_ms};
use crate::task::{block_current_and_run_next, current_task};
use lazy_static::*;
use alloc::vec::Vec;


pub fn sys_sleep(ms: usize) -> isize {
    let expire_ms = get_time_ms() + ms;
    let task = current_task().unwrap();
    add_timer(expire_ms, task);
    block_current_and_run_next();
    0
}

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
