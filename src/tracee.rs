use std::process::Command;

fn main() {
    println!("tracee started");
    Command::new("./test-script.sh")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
