#include <stdio.h>
#include <stdlib.h>

int foo = 42;

// Suggestion: implement this feature after dlsym/dlopen works (2)
void my_atexitt() { puts("side module atexit"); }

void direct_function_call() { puts("Test successful"); }
void (*indirect_function_call)() = *direct_function_call;

// Suggestion: implement this feature after dlsym/dlopen works (1)
__attribute__((constructor)) static void ctor() {
  puts("side module ctor");
  //   atexit(my_atexitt);
}