extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/exec_child.c")
        .compile("tracer_c");
}
