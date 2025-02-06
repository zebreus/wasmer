#include <assert.h>
#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  void *handle = dlopen("side.wasm", RTLD_NOW);
  if (!handle) {
    printf("dlopen failed: %s\n", dlerror());
    return 1;
  }
  printf("handle: %p\n", handle);
  int *foo = (int *)dlsym(handle, "foo");
  if (!foo) {
    printf("dlsym failed: %s\n", dlerror());
    return 1;
  }
  printf("foo = %d\n", *foo);
  assert(*foo == 42);
  exit(0);
}