extern crate libc;
extern crate mine;

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

use mine::registers::new_user_regs_struct;
use mine::syscall::get_syscall;

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
    let mut registers: user_regs_struct = new_user_regs_struct();
    #[allow(deprecated)]
    unsafe {
        ptrace::ptrace(
            Request::PTRACE_GETREGS,
            child,
            null_mut(),
            (&mut registers) as *mut _ as *mut c_void,
        )?;
    }
    let syscall = get_syscall(registers);
    println!(
        "{}({}, {}, {}, {}, {}, {})",
        syscall,
        registers.rdi,
        registers.rsi,
        registers.rdx,
        registers.r10,
        registers.r8,
        registers.r9
    );
    match syscall {
        "__NR_execve" => {
            let filename = registers.rdi as *const c_char;
            if filename != std::ptr::null() {
                println!("child process spawned: {:?}", unsafe {
                    CStr::from_ptr(filename)
                });
            }
        }
        _ => {}
    }
    Ok(())
}
