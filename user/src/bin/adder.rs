#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{ fork, getpid, waitpid, get_share, set_share, sleep, semaphore_create, semaphore_up,semaphore_down};

#[no_mangle]
pub fn main() -> i32 {
    let sema:isize = semaphore_create(1);
    let pid = fork();
    if pid == 0 {
        semaphore_down(sema as usize);
        println!(
            "child: share's old value is {}. sema_id:{} ",
            get_share(),sema
        );
        set_share();
        println!(
            "child: share's new value is {}.\n ",
            get_share()
        );
        semaphore_up(sema as usize);
        100
    } else {
        let mut exit_code: i32 = 0;
        semaphore_down(sema as usize);
         println!(
             "parent: share's old value is {}. sema_id:{}",
            get_share(),sema
        );
        set_share();
        println!(
            "parent: share's new value is {}.\n ",
            get_share()
        );
        semaphore_up(sema as usize);
        waitpid(pid as usize, &mut exit_code);
        0
    }
}
