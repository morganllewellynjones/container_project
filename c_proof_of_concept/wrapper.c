#define _GNU_SOURCE
#include <linux/sched.h>
#include <sched.h>
#include <sys/syscall.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>


int main() {

  int status = chroot("container");
  printf("errno is: %s\n", strerror(errno));
  printf("status is: %d\n", status);
  
  unshare(CLONE_NEWPID | CLONE_NEWNET | CLONE_NEWNS);

  pid_t child = fork();
  if (child == 0) {
    printf("child pid is %d\n", getpid());
  }

  sleep(6000);
}
