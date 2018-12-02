#include <unistd.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <sys/ptrace.h>
#include <sys/wait.h>
#include <sys/user.h>

int exec_child() {
  printf("started exec_child...\n");
  const char* filename = "./test.sh";
  char* empty[] = { NULL };
  int output = execve(filename, empty, empty);
  printf("failed: %s\n", strerror(errno));
  return 0;
}

int main() {
  printf("started...\n");
  int id = fork();
  if (id < 0) {
    printf("failed!\n");
  } else if (id == 0) {
    printf("I'm the child process, lalalala!!!\n");
    ptrace(PTRACE_TRACEME, NULL, NULL, NULL);
    raise(SIGSTOP);
    exec_child();
  } else {
    int child_pid = id;
    int status = 1;
    while (!WIFEXITED(status)) {
      waitpid(child_pid, &status, 0);
      printf("status: %d\n", status);
      if (WIFSTOPPED(status)) {
        // siginfo_t siginfo;
        // ptrace(PTRACE_GETSIGINFO, child_pid, &siginfo, NULL);
        struct user_regs_struct regs;
        ptrace(PTRACE_GETREGS, child_pid, 0, &regs);
        printf("syscall: %llu\n", regs.rax);
        // printf("stop sig: %s\n", strsignal(WSTOPSIG(status)));
        ptrace(PTRACE_SYSCALL, child_pid, NULL, NULL);
      }
    }
  }
}
