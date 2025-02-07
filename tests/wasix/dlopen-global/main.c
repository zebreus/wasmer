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

  void (**indirect_function_call)();
  indirect_function_call = dlsym(handle, "indirect_function_call");
  (**indirect_function_call)();

  void (*direct_function_call)();
  direct_function_call = dlsym(handle, "direct_function_call");
  // printf("print_test as ptr: %p\n", print_test);
  // printf("print_test as size_t: %u\n", print_test);
  (*direct_function_call)();

  exit(0);
}