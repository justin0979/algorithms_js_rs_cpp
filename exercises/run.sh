#!/bin/bash

# Quickly run javascript tests by providing the directory and test.js file.
# 
# To run:
#   ./run.sh <DIRECTORY>

FILE=$1
FILE_LEN=${#FILE}
LAST_CHAR=${FILE:FILE_LEN-1}

# Check if last char is "/", if yes, move on, if not, add "/"
if [[ ${FILE:FILE_LEN-1:1} == *[a-z]* ]]; then
  FILE=${FILE}/
fi

npx jest ${FILE}test.js --watch
