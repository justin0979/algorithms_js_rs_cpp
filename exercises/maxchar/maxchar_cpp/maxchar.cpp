#include "maxchar.h"
#include<tuple>
#include<algorithm>

char maxchar(std::string str) {
  std::sort(str.begin(), str.end());
  int count = 0;
  char ch = str[0];
  std::tuple<int, char> maxchar = std::make_tuple(1, ch);

  for(size_t i = 0; i < str.size(); i++) {
    if(ch == str[i]) {
      count += 1;

      if(std::get<0>(maxchar) < count) {
        std::get<0>(maxchar) = count;
        std::get<1>(maxchar) = str[i];
      }
    } else {
      count = 0;
      ch = str[i];
    }
  }

  return std::get<1>(maxchar);
}
