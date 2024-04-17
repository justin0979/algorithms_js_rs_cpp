#include<iostream>
#include<vector>
#include "chunk.h"

std::vector<std::vector<int>> chunk(std::vector<int> array, int size) {
  std::vector<std::vector<int>> chunks ;
  int count = 0;

  // Loop to add the number of fully filled chunks
  for(int i = 0; i < array.size()/size; i++) {
    std::vector<int> temp_chunk;
    
    // Loop creates the chunk
    for(int j = 0; j < size; j++) {
      temp_chunk.push_back(array[count]);
      count++;
    }
    chunks.push_back(temp_chunk);
    temp_chunk.clear();
  }

  // Put remainder of values in last chunk
  if(count < array.size()) {
    std::vector<int> temp_chunk;
    while(count < array.size()) {
      temp_chunk.push_back(array[count]);
      count++;
    }

    chunks.push_back(temp_chunk);
  }

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

