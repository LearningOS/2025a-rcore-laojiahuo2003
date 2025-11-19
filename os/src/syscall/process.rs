//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, get_syscall_count, suspend_current_and_run_next},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
// pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
//     trace!("kernel: sys_trace");
//     -1
// }
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace: request={}, id={:#x}, data={:#x}", trace_request, id, data);
    
    match trace_request {
        0 => {
            // 如果 trace_request 为 0，则 id 应被视作 *const u8 ，表示读取当前任务 id 地址处一个字节的无符号整数值。此时应忽略 data 参数。返回值为 id 地址处的值。
            unsafe {
                if true {
                    let value = *(id as *const u8);
                    value as isize
                } else {
                    -1
                }
            }
        }
        1 => {
            // 如果 trace_request 为 1，则 id 应被视作 *mut u8 ，表示写入 data （作为 u8，即只考虑最低位的一个字节）到该用户程序 id 地址处。返回值应为0。
             // 写入一个字节到用户空间地址 id
            unsafe {
                if true {
                    *(id as *mut u8) = (data & 0xFF) as u8;
                    0
                } else {
                    -1
                }
            }
        }
        2 => {
            // 如果 trace_request 为 2，表示查询当前任务调用编号为 id 的系统调用的次数，返回值为这个调用次数。本次调用也计入统计 。
            get_syscall_count(id) as isize
        }
        _ => {
            // 否则，忽略其他参数，返回值为 -1。
            -1
        }
    }
}
