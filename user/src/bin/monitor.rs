#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{ fork, getpid, waitpid, sleep, semaphore_create};
use user_lib::Vec;
#[no_mangle]
use user_lib::Monitor;


pub fn main() -> i32 {
    let mut children: Vec<isize> = Vec::new();
    let mut monitor: Monitor = Monitor::new();
    monitor.init_monitor();
    // 创建3个生产者子进程
    for _ in 0..3 {
        let pid = fork();
        if pid == 0 {
            // 这是子进程，执行生产者逻辑
            for _ in 0..10 {
                monitor.append(getpid());
                sleep(1);
            }
            return 0; // 结束生产者子进程
        } else {
            // 父进程中记录子进程的PID
            children.push(pid);
        }
    }

    // 创建3个消费者子进程
    for _ in 0..3 {
        let pid = fork();
        if pid == 0 {
            // 这是子进程，执行消费者逻辑
            for _ in 0..10 {
                monitor.take(getpid());
                sleep(1);
            }
            return 0; // 结束消费者子进程
        } else {
            // 父进程中记录子进程的PID
            children.push(pid);
        }
    }
    // 父进程等待所有子进程结束
    for child_pid in children {
        let mut exit_code: i32 = 0;
        waitpid(child_pid as usize, &mut exit_code);
    }
    0
}
