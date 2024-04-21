#include<iostream>
#include "maxchar.h"

int main() {
  std::string input = "bzcdefghijklmnzzzzz";
  char solution = 'z';
  if(solution == maxchar(input)) {
    std::cout << "The max char in \"" << input << "\" is " << solution << std::endl;
  } else {
    std::cout << "Your program is busted." << std::endl;
  }
  return 0;
}
