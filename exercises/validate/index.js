// --- Directions
// Given a node, validate the binary search tree,
// ensuring that every node's left hand child is
// less than the parent node's value, and that
// every node's right hand child is greater than
// the parent

function validate(node, min = null, max = null) {}

module.exports = validate;
//  let n = node;
//
//  console.log("node.data =", node.data);
//
//  let left = false;
//  let right = false;
//  if (!min && !max) {
//    if (
//      n.left &&
//      n.right &&
//      n.left.data < n.data &&
//      n.right.data > n.data
//    ) {
//      left = validate(n.left, n.left, n);
//      if (left) {
//        console.log("ANY RIGHT");
//        right = validate(n.right, n, n.right);
//      } else {
//        console.log("LEFT =", left, "WAS IT HERE");
//        return false;
//      }
//    }
//  } else {
//    // LEFT OF ROOT
//    if (
//      n.left &&
//      n.right &&
//      n.left.data < n.data &&
//      n.right.data > n.data &&
//      n.right.data < max.data &&
//      n.left.data < min.data
//    ) {
//      left = validate(n.left, n.left, n);
//      if (left) {
//        right = validate(n.right, n, n.right);
//      } else {
//        return false;
//      }
//    }
//    // RIGHT OF ROOT
//    else if (
//      n.left &&
//      n.right &&
//      n.left.data < n.data &&
//      n.right.data > n.data &&
//      n.right.data > max.data &&
//      n.left.data > min.data
//    ) {
//      left = validate(n.left, min, n.left);
//      if (left) {
//        right = validate(n.right, min, n.right);
//      } else {
//        return false;
//      }
//    }
//    // LEFT ONLY
//    else if (n.left && !n.right && n.left.data < n.data) {
//      return validate(n.left, n.left, n);
//    }
//    // RIGHT ONLY
//    else if (n.right && !n.left && n.right.data > n.data) {
//      return validate(n.right, min, n.right);
//    }
//    // LEAF
//    else if (n.left === null && n.right === null) {
//      console.log("LEAF node, node =", node.data);
//      return true;
//    } else {
//      console.log(
//        "FAILURE IN ELSE, node =",
//        n.data,
//        "n.left =",
//        n.left,
//        "n.right =",
//        n.right,
//      );
//      return false;
//    }
//  }
//  return left && right;
