all: tracer tracee

tracer: tracer.c
	gcc tracer.c -o tracer

tracee: tracee.c
	gcc tracee.c -o tracee
