extern crate cc;

fn main() {
    cc::Build::new().file("src/tracer.c").compile("tracer_c");
}
