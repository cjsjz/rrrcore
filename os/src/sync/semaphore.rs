use crate::sync::UPSafeCell;
use alloc::{collections::VecDeque, sync::Arc};
use crate::task::{block_current_and_run_next, current_task, wakeup_task, TaskControlBlock};

/// 信号量实现
pub struct Semaphore {
    /// 信号量内部数据
    pub inner: UPSafeCell<SemaphoreInner>,
}

pub struct SemaphoreInner {
    pub count: isize,
    pub wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl Semaphore {
    /// 创建一个信号量，初始值为 `count`。
    pub fn new(res_count: usize) -> Self {
        Self {
            inner: unsafe {
                UPSafeCell::new(SemaphoreInner {
                    count: res_count as isize,
                    wait_queue: VecDeque::new(),
                })
            },
        }
    }

    /// 增加信号量的计数，并尝试唤醒等待的任务。
    pub fn up(&self) {
        let mut inner = self.inner.exclusive_access();
        inner.count += 1;
        if inner.count <= 0 {
            if let Some(task) = inner.wait_queue.pop_front() {
                wakeup_task(task);
            }
        }
    }

    /// 减少信号量的计数，如果计数为负，则阻塞当前任务。
    pub fn down(&self) {
        let mut inner = self.inner.exclusive_access();
        inner.count -= 1;
        if inner.count < 0 {
            if let Some(task) = current_task() {
                inner.wait_queue.push_back(task);
                drop(inner); // 释放锁，避免死锁
                block_current_and_run_next();
            } else {
                // 处理 current_task() 返回 None 的情况
                panic!("No current task to block!");
            }
        }
    }
}
