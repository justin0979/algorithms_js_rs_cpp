const fromLast = require("./index");
const L = require("./linkedlist");
const List = L.LinkedList;
const Node = L.Node;

test("fromLast is a function", () => {
  expect(typeof fromLast).toEqual("function");
});

test("fromLast returns the node n elements from the end", () => {
  const l = new List();

  l.insertLast("a");
  l.insertLast("b");
  l.insertLast("c");
  l.insertLast("d");
  l.insertLast("e");

  expect(fromLast(l, 3).data).toEqual("b");
});

test("fromLast returns the head element from list of size 2", () => {
  const l = new List();

  l.insertLast("a");
  l.insertLast("b");

  expect(fromLast(l, 1).data).toEqual("a");
});

test("fromLast returns the last element from list when n = 0", () => {
  const l = new List();

  l.insertLast("a");
  l.insertLast("b");
  l.insertLast("c");
  l.insertLast("d");
  l.insertLast("e");
  l.insertLast("f");

  expect(fromLast(l, 0).data).toEqual("f");
});

test("fromLast returns the only element in list", () => {
  const l = new List();

  l.insertLast("a");

  expect(fromLast(l, 0).data).toEqual("a");
});
