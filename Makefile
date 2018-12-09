all: tracer tracee

tracer: tracer.c
	gcc tracer.c -o tracer

tracee: tracee.rs
	rustc tracee.rs
