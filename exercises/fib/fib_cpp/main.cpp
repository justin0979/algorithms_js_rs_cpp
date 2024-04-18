#include<iostream>
#include "fib.h";

int main(int argc, char **argv) {
  int n = 42;
  int ans = fib(n);

  std::cout << "The fibonicci number in spot " << n << " is " << ans << "." << std::endl;

  return 0;
}
