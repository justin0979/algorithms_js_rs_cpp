#include<vector>
#include "fib.h"

int fib(int n) {
  std::vector<int> fib_seq = {0,1};

  for(int i = 0; i <= n; i++) {
    fib_seq.push_back(fib_seq[i - 1] + fib_seq[i - 2]);
  }

  return fib_seq[n];
}
