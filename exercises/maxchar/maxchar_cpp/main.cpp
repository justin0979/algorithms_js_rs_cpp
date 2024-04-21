#include<iostream>
#include "maxchar.h"

void print_maxchar_run(std::string input, char solution);

int main() {
  print_maxchar_run("bzcdefghijklmnzzzzz", 'z');
  print_maxchar_run("abcdefghijklmnaaaaa", 'a');
  print_maxchar_run("ab1c1d1e1f1g1", '1');
  print_maxchar_run("cqqyyyabcabcddd123333zyyrrzyyqaayyyyyyyyyayz9", 'y');
  return 0;
}

void print_maxchar_run(std::string input, char solution) {
  if(solution == maxchar(input)) {
    std::cout << "The max char in \"" << input << "\" is " << "'" <<
      solution << "'." <<  std::endl;
  } else {
    std::cout << "Your program is busted, it gave the wrong max character." << std::endl;
  }
}
