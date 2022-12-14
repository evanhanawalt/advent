const fs = require("fs");
const input = fs.readFileSync("input.txt", "utf8").split("\n\n");

// this function is a mess but it works
const checkPackets = (left, right) => {
  for (let i = 0; i < left.length; i++) {
    if (i >= right.length) {
      // right runs out of items before left = false
      return false;
    } else if (typeof left[i] === "number" && typeof right[i] === "number") {
      // left > right = false; right > left = true; right===left = keep going
      if (left[i] > right[i]) {
        return false;
      } else if (left[i] < right[i]) {
        return true;
      }
    } else if (typeof left[i] === "object" && typeof right[i] === "object") {
      // if both are arrays, do recursion
      // undefined value means the test was inconclusive
      let subVal = checkPackets(left[i], right[i]);
      if (subVal !== undefined && subVal === true) {
        return true;
      } else if (subVal !== undefined && subVal === false) {
        return false;
      }
    } else if (typeof left[i] === "object") {
      //make an array of the right number to compare to left array
      let subVal = checkPackets(left[i], [right[i]]);
      if (subVal !== undefined && subVal === true) {
        return true;
      } else if (subVal !== undefined && subVal === false) {
        return false;
      }
    } else if (typeof right[i] === "object") {
      //make an array of the left number to compare to right array
      let subVal = checkPackets([left[i]], right[i]);
      if (subVal !== undefined && subVal === true) {
        return true;
      } else if (subVal !== undefined && subVal === false) {
        return false;
      }
    }
  }
  if (left.length < right.length) {
    // left ran out of items before right
    return true;
  }
};
let jsSorter = (a, b) => {
  let value = checkPackets(a, b);
  if (value === undefined) return -1;
  else if (value) return -1;
  else return 1;
};
let allPackets = [[[2]], [[6]]];
let sum = 0;
for (let i = 0; i < input.length; i++) {
  // This next line of code is an atrocity
  let [left, right] = input[i].split("\n").map((value) => eval(value));
  allPackets.push(left);
  allPackets.push(right);
}
allPackets.sort(jsSorter);
const arrayEquals = (a, num) => {
  return (
    Array.isArray(a) &&
    a.length === 1 &&
    Array.isArray(a[0]) &&
    a[0].length === 1 &&
    a[0][0] === num
  );
};

let first = null;
let last = null;

for (let i = 0; i < allPackets.length; i++) {
  if (arrayEquals(allPackets[i], 2)) {
    first = 1 + i;
  } else if (arrayEquals(allPackets[i], 6)) {
    last = 1 + i;
  }
}

console.log("decoderKey:", first * last);
