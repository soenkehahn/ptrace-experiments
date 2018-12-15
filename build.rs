extern crate cc;

fn main() {
    cc::Build::new().file("tracer.c").compile("tracer_c");
}
