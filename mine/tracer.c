#include <unistd.h>
#include <assert.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <sys/ptrace.h>
#include <sys/wait.h>
#include <sys/user.h>
#include <asm/unistd.h>

int exec_child() {
  const char* filename = "./tracee";
  char* empty[] = { NULL };
  execve(filename, empty, empty);
  return 0;
}

void debug_syscall_enter_stop(int child_pid) {
  struct user_regs_struct regs;
  ptrace(PTRACE_GETREGS, child_pid, 0, &regs);
  unsigned long long int syscall = regs.orig_rax;

  if (syscall == __NR_execve) {
    printf("exec\n");
    unsigned long long int first_argument = regs.rdi;
    if (first_argument != 0) {
      const char* filename = (char*) first_argument;
      printf("child process spawned: %s\n", filename);
    }
  }
}

int main_(int id) {
  if (id == 0) {
    // child
    ptrace(PTRACE_TRACEME, NULL, NULL, NULL);
    raise(SIGSTOP);
    exec_child();
  } else {
    // parent
    int child_pid = id;
    while (1) {
      int status;
      waitpid(child_pid, &status, 0);
      if (WIFEXITED(status)) {
        break;
      }
      debug_syscall_enter_stop(child_pid);
      ptrace(PTRACE_SYSCALL, child_pid, NULL, NULL);
    }
  }
  return 0;
}
