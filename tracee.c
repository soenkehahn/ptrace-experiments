#include <stdio.h>
#include <sys/wait.h>
#include <unistd.h>

int main() {
  printf("tracee started\n");
  int id = fork();
  if (id == 0) {
    const char* filename = "./test-script.sh";
    char* empty[] = { NULL };
    int output = execve(filename, empty, empty);
  }
  wait(0);
  return 0;
}
