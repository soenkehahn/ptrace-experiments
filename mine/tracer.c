#include <stdio.h>
#include <unistd.h>

int exec_child() {
  const char* filename = "./tracee";
  char* empty[] = { NULL };
  execv(filename, empty);
  return 0;
}
