#include<iostream>
#include<vector>
#include "chunk.h"

int main(int argc, char** argv) {
  std::vector chunks_5 = {1,2,3,4,5};
  const int size_2 = 2;
  std::vector<std::vector<int>> answer_5_2 = chunk(chunks_5, size_2);
  print_answer(chunks_5, 2, answer_5_2);
  std::cout << std::endl;

  std::vector chunks_9 = {1,2,3,4,5, 6, 7, 8, 9};
  const int size_5 = 5;
  std::vector<std::vector<int>> answer_9_5 = chunk(chunks_9, size_5);
  print_answer(chunks_9, size_5, answer_9_5);
  std::cout << std::endl;

  const int size_3 = 3;
  std::vector<std::vector<int>> answer_9_3 = chunk(chunks_9, size_3);
  print_answer(chunks_9, size_3, answer_9_3);

  return 0;
}
