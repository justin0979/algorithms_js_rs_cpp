#include<iostream>
#include<vector>

std::vector<std::vector<int>> chunk(std::vector<int> array, int size);
void print_1d_array(std::vector<int> v);
void print_answer(std::vector<int> array, int size, std::vector<std::vector<int>> answer);

int main(int argc, char** argv) {
  std::vector chunks = {1,2,3,4,5};
  const int size_2 = 2;
  std::vector<std::vector<int>> answer = chunk(chunks, size_2);
  print_answer(chunks, 2, answer);

  std::vector chunks9 = {1,2,3,4,5, 6, 7, 8, 9};
  const int size_5 = 5;
  std::vector<std::vector<int>> answer9 = chunk(chunks9, size_5);
  print_answer(chunks9, size_5, answer9);

  return 0;
}

std::vector<std::vector<int>> chunk(std::vector<int> array, int size) {
  std::vector<std::vector<int>> chunks ;
  int count = 0;

  for(int i = 0; i < array.size()/size; i++) {
    std::vector<int> temp_chunk;
    
    for(int j = 0; j < size; j++) {
      temp_chunk.push_back(array[count]);
      count++;
    }
    chunks.push_back(temp_chunk);
    temp_chunk.clear();
  }

  std::vector<int> temp_chunk;
  while(count < array.size()) {
    temp_chunk.push_back(array[count]);
    count++;
  }

  chunks.push_back(temp_chunk);

  return chunks;
}

void print_1d_array(std::vector<int> v) {
  std::cout << "[ ";
  for(auto i : v) {
    std::cout << i << " ";
  }
  std::cout << "]" <<std::endl;
}

void print_answer(std::vector<int> array, int size, std::vector<std::vector<int>> answer) {
  std::cout << "Array: ";
 print_1d_array(array); 
 std::cout << " with chunks' size of " << size << " will become\n[ ";
 for(int i = 0; i < answer.size(); i++) {
   std::cout << "[ ";
   for(auto j:answer[i]) {
     std::cout << j << " ";
   }
   std::cout << "] ";
 }
 std::cout << "]" << std::endl;
}
