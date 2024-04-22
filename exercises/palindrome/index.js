// --- Directions
// Given a string, return true if the string is a palindrome
// or false if it is not.  Palindromes are strings that
// form the same word if it is reversed. *Do* include spaces
// and punctuation in determining if the string is a palindrome.
// --- Examples:
//   palindrome("abba") === true
//   palindrome("abcdefg") === false

function palindrome(str) {
  let s_one = str
    .slice(0, str.length / 2)
    .split("")
    .reverse()
    .join("");
  let s_two;

  if (str.length % 2 == 0) {
    s_two = str.slice(str.length / 2);
  } else {
    s_two = str.slice(str.length / 2 + 1);
  }
  if (s_one === s_two) {
    return true;
  } else {
    return false;
  }
}

module.exports = palindrome;
