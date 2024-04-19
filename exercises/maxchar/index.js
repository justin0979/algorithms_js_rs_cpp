// --- Directions
// Given a string, return the character that is most
// commonly used in the string.
// --- Examples
// maxChar("abcccccccd") === "c"
// maxChar("apple 1231111") === "1"

// Iterative solution with no object to hold chars and char occurrences;
function maxChar(str) {
  str = str.split("").sort();

  let counter = 0;
  let maxCount = 1;
  let currCh = str[0];
  let maxCh = str[0];

  for (let i = 0; i < str.length; i++) {
    if (str[i] === currCh) {
      counter += 1;
      if (maxCount < counter) {
        maxCount = counter;
        maxCh = str[i];
      }
    } else {
      currCh = str[i];
      counter = 1;
    }
  }

  return maxCh;
}

// Iterative solution with object to hold chars and char occurrences;
function maxChar_w_obj_lookup(str) {
  const charTracker = {};
  let max = 0;
  let maxCh = str[0];

  for (let i = 0; i < str.length; i++) {
    charTracker[str[i]] = charTracker[str[i]] + 1 || 1;
    if (max < charTracker[str[i]]) {
      max = charTracker[str[i]];
      maxCh = str[i];
    }
  }
  return maxCh;
}

module.exports = maxChar;
