extern crate libc;

use libc::c_char;
use libc::user_regs_struct;
use nix::sys::ptrace;
use nix::sys::ptrace::Request;
use nix::sys::signal::raise;
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execv, fork, ForkResult, Pid};
use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::null_mut;

extern "C" {
    fn exec_child();
}

#[derive(Debug)]
struct AppError(String);

impl From<std::ffi::NulError> for AppError {
    fn from(error: std::ffi::NulError) -> AppError {
        AppError(format!("{:?}", error))
    }
}

impl From<nix::Error> for AppError {
    fn from(error: nix::Error) -> AppError {
        AppError(format!("{:?}", error))
    }
}

fn main() -> Result<(), AppError> {
    match fork()? {
        ForkResult::Child => {
            ptrace::traceme()?;
            raise(Signal::SIGSTOP)?;
            if false {
                execv(&CString::new("./tracee")?, &vec![])?;
            } else {
                unsafe { exec_child() };
            }
        }
        ForkResult::Parent { child } => {
            wait_for_sigstop(child)?;
            loop {
                ptrace::syscall(child)?;
                match waitpid(child, None)? {
                    WaitStatus::Exited(..) => {
                        break;
                    }
                    _ => {}
                }
                debug_syscall(child)?;
            }
        }
    };
    Ok(())
}

fn wait_for_sigstop(child: Pid) -> Result<(), AppError> {
    match waitpid(child, None)? {
        WaitStatus::Stopped(_, Signal::SIGSTOP) => {}
        _ => panic!("SIGSTOP expected"),
    }
    Ok(())
}

fn debug_syscall(child: Pid) -> Result<(), AppError> {
    let mut regs: user_regs_struct = new_user_regs_struct();
    #[allow(deprecated)]
    unsafe {
        ptrace::ptrace(
            Request::PTRACE_GETREGS,
            child,
            null_mut(),
            (&mut regs) as *mut _ as *mut c_void,
        )?;
    }
    let syscall = regs.orig_rax;
    if syscall == __NR_EXECVE {
        println!("exec");
        let filename = regs.rdi as *const c_char;
        if filename != std::ptr::null() {
            println!("child process spawned: {:?}", unsafe {
                CStr::from_ptr(filename)
            });
        }
    };
    Ok(())
}

const __NR_EXECVE: u64 = 59;

fn new_user_regs_struct() -> user_regs_struct {
    user_regs_struct {
        r15: 0,
        r14: 0,
        r13: 0,
        r12: 0,
        rbp: 0,
        rbx: 0,
        r11: 0,
        r10: 0,
        r9: 0,
        r8: 0,
        rax: 0,
        rcx: 0,
        rdx: 0,
        rsi: 0,
        rdi: 0,
        orig_rax: 0,
        rip: 0,
        cs: 0,
        eflags: 0,
        rsp: 0,
        ss: 0,
        fs_base: 0,
        gs_base: 0,
        ds: 0,
        es: 0,
        fs: 0,
        gs: 0,
    }
}
