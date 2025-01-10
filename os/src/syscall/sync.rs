use crate::sync::{Semaphore,UPSafeCell,Monitor};
use crate::timer::{add_timer, get_time_ms};
use crate::task::{block_current_and_run_next, current_task};
use alloc::sync::Arc;
use lazy_static::*;
use alloc::vec::Vec;


struct Circlebuf {
    buf: [i32; 10],
    read_pos: usize,
    write_pos: usize,
    buf_count: usize,
}
impl Circlebuf {
    pub fn new() -> Self {
        Self {
            buf: [0; 10],
            read_pos: 0,
            write_pos: 0,
            buf_count: 0,
        }
    }
    pub fn push(&mut self, pid: usize) {
        self.buf[self.write_pos] = pid as i32;
        self.buf_count += 1;
        sys_sleep(1);
        println!("processer {} push valuse {} to {}",pid,pid,self.write_pos);
        self.write_pos = (self.write_pos + 1) % 10;
    }
    pub fn pop(&mut self, pid: usize)  {
        if self.buf[self.read_pos] != 0 {
            println!("customer {} pop valuse {} from {}",pid,self.buf[self.read_pos],self.read_pos);
            self.buf[self.read_pos] = 0;
            self.buf_count -= 1;
            sys_sleep(1);
            self.read_pos = (self.read_pos + 1) % 10;
        }else {
            println!("customer pop failed");
        }
    }
    pub fn get_buf_count(&self) -> usize {
        self.buf_count
    }
}

pub fn sys_sleep(ms: usize) -> isize {
    let expire_ms = get_time_ms() + ms;
    let task = current_task().unwrap();
    add_timer(expire_ms, task);
    block_current_and_run_next();
    0
}

lazy_static! {
    static ref CIRCLE_BUF: UPSafeCell<Circlebuf> = unsafe { UPSafeCell::new(Circlebuf ::new()) };
}

lazy_static! {
    pub static ref SEMAPHOR_VEC: UPSafeCell<Vec<Option<Arc<Semaphore>>>> = unsafe { UPSafeCell::new(Vec::new()) };
}

lazy_static! {
    pub static ref MONITOR : UPSafeCell<Arc<Monitor>> = unsafe { UPSafeCell::new(Arc::new(Monitor::new())) };
}

/// Create a new semaphore with the given initial resource count
pub fn sys_semaphore_create(res_count: usize) -> isize {
    let sema = Semaphore::new(res_count);
    let mut sema_vec = SEMAPHOR_VEC.exclusive_access();
    sema_vec.push(Some(Arc::new(sema)));
    sema_vec.len() as isize
}
///Realese a semaphore with the given id
pub fn sys_semaphore_up(sem_id: usize) -> isize {
    let sema_vec = SEMAPHOR_VEC.exclusive_access();
    //if sem_id > sema_vec.len() {
     //   return -1;
    //}
    let sema = Arc::clone(sema_vec[sem_id-1].as_ref().unwrap());
    drop(sema_vec);
    sema.up();
    //println!("sem_id is {}",sem_id);
    0
}
///Wait for a semaphore with the given id
pub fn sys_semaphore_down(sem_id: usize) -> isize {
    let sema_vec = SEMAPHOR_VEC.exclusive_access();
    //if sem_id > sema_vec.len() {
    //    return -1;
    //}
    let sema = Arc::clone(sema_vec[sem_id-1].as_ref().unwrap());
    drop(sema_vec);
    sema.down();
    0
}
///Get the value of the circular buffer
pub fn sys_get(pid : usize) -> isize {
    CIRCLE_BUF.exclusive_access().pop(pid);
    0
}
///Set the value of the circular buffer
pub fn sys_set(pid : usize) -> isize {
    CIRCLE_BUF.exclusive_access().push(pid);
    0
}
///Get the value of the circular buffer count
pub fn sys_get_buf_count() -> isize {
    CIRCLE_BUF.exclusive_access().get_buf_count() as isize
}

///Enter a monitor
pub fn sys_monitor_enter() -> isize {
    let monitor = MONITOR.exclusive_access();
    let monitor_clone = Arc::clone(&monitor);
    drop(monitor);
    monitor_clone.enter();
    0
}

///Leave a monitor
pub fn sys_monitor_leave() -> isize {
    let monitor = MONITOR.exclusive_access();
    let monitor_clone = Arc::clone(&monitor);
    drop(monitor);
    monitor_clone.leave();
    0
}

///Wait for a monitor
pub fn sys_monitor_wait(x_sem_id: usize) -> isize {
    let monitor = MONITOR.exclusive_access();
    let monitor_clone = Arc::clone(&monitor);
    drop(monitor);
    monitor_clone.wait(x_sem_id);
    0
}

///Signal a monitor
pub fn sys_monitor_signal(x_sem_id: usize) -> isize {
    let monitor = MONITOR.exclusive_access();
    let monitor_clone = Arc::clone(&monitor);
    drop(monitor);
    monitor_clone.signal(x_sem_id);
    0
}

///Set value of the condition variable
pub fn sys_set_condition_var(x_sem_id: usize, value: usize) -> isize {
    let monitor = MONITOR.exclusive_access();
    let monitor_clone = Arc::clone(&monitor);
    drop(monitor);
    monitor_clone.set_value(x_sem_id, value as isize);
    0
}