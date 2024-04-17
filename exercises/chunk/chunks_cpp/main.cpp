#include<iostream>
#include<vector>

std::vector<std::vector<int>> chunk(std::vector<int> array, int size);

int main(int argc, char** argv) {
  std::vector chunks = {1,2,3,4,5};

  std::vector<std::vector<int>> answer = chunk(chunks, 2);

  std::cout << "[ ";
  for(int i = 0; i < answer.size(); i++) {
    std::cout << "[ ";
    for(int j = 0; j < answer[i].size(); j++) {
      std::cout << j << " ";
    }
    std::cout << "] ";
  }
  std::cout << "]" << std::endl;

  return 0;
}

std::vector<std::vector<int>> chunk(std::vector<int> array, int size) {
  std::vector<std::vector<int>> chunks ;



  return chunks;
}
