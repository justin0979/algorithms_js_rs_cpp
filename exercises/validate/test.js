const Node = require("./node");
const validate = require("./index");

test("Validate recognizes a valid BST with only left side", () => {
  //  console.log("****************\nBegin ONE");
  const n = new Node(10);
  n.insert(9);
  n.insert(5);
  n.insert(0);

  expect(validate(n)).toEqual(true);
  // console.log("End ONE\nxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n");
});

test("Validate recognizes a valid BST with only right side", () => {
  //  console.log("****************\nBegin ONE");
  const n = new Node(10);
  n.insert(15);
  n.insert(20);
  n.insert(25);

  expect(validate(n)).toEqual(true);
  // console.log("End ONE\nxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n");
});

test("Validate recognizes a valid BST", () => {
  //  console.log("****************\nBegin ONE");
  const n = new Node(10);
  n.insert(5);
  n.insert(15);
  n.insert(0);
  n.insert(20);
  n.insert(14);
  n.insert(6);

  expect(validate(n)).toEqual(true);
  // console.log("End ONE\nxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n");
});

test("Validate recognizes a much larger valid BST", () => {
  //console.log("****************\nBegin ONE");
  const n = new Node(50);
  n.insert(30);
  n.insert(70);
  n.insert(20);
  n.insert(0);
  n.insert(40);
  n.insert(25);
  n.insert(21);
  n.insert(28);
  n.insert(35);
  n.insert(45);
  n.insert(60);
  n.insert(65);
  n.insert(50);
  n.insert(63);
  n.insert(69);
  n.insert(90);

  expect(validate(n)).toEqual(true);
  // console.log("End ONE\nxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n");
});

test("Validate recognizes an invalid BST, from value greater than root on left side", () => {
  const n = new Node(10);
  n.insert(5);
  n.insert(15);
  n.insert(0);
  n.insert(20);
  n.insert(14);
  n.left.left.right = new Node(999);

  expect(validate(n)).toEqual(false);
});

test("Validate recognizes an invalid BST, from value less than root on right side", () => {
  const n = new Node(10);
  n.insert(5);
  n.insert(15);
  n.insert(0);
  n.insert(20);
  n.right.left = new Node(9);

  expect(validate(n)).toEqual(false);
});

test("Validate recognizes a invalid BST, from value less than root on right side.", () => {
  //  console.log("****************\nBegin THREE");
  const n = new Node(10);
  n.insert(5);
  n.insert(4);
  n.insert(15);
  n.insert(0);
  n.insert(14);
  n.insert(20);
  n.right.right.left = new Node(1);

  expect(validate(n)).toEqual(false);
  // console.log("End THREE\nxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n");
});

test("Invalid, value less than root but greater than parent node's parent", () => {
  // console.log("****************\nBegin FOUR");
  const n = new Node(10);
  n.insert(5);
  n.insert(15);
  n.insert(0);
  n.insert(20);
  n.left.left.right = new Node(6);

  expect(validate(n)).toEqual(false);
  //  console.log("End FOUR\nxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n");
});
