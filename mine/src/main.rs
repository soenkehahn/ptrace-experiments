extern crate libc;

use libc::c_int;
use nix::unistd::{fork, ForkResult};

extern "C" {
    fn main_(id: c_int);
}

fn main() -> Result<(), nix::Error> {
    let id = match fork()? {
        ForkResult::Child => 0,
        ForkResult::Parent { child } => child.as_raw(),
    };
    unsafe {
        main_(id);
    }
    Ok(())
}
