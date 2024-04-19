const maxChar = require("./index");

test("maxChar function exists", () => {
  expect(typeof maxChar).toEqual("function");
});

test("expect most frequent character of 'bzcdefghijklmnzzzzz' to be z", () => {
  expect(maxChar("bzcdefghijklmnzzzzz")).toEqual("z");
});

test("a", () => {
  expect(maxChar("a")).toEqual("a");
});

test("Works with numbers in the string", () => {
  expect(maxChar("ab1c1d1e1f1g1")).toEqual("1");
});

test("12yyyabcabcddd123333zyyrrzyyqaayyyyyyyyyayz9", () => {
  expect(
    maxChar("12yyyabcabcddd123333zyyrrzyyqaayyyyyyyyyayz9"),
  ).toEqual("y");
});
