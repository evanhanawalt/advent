const fs = require("fs");
const input = fs.readFileSync("input.txt", "utf8").split("\n\n");

const checkPackets = (left, right) => {
    for (let i = 0; i < left.length; i++) {
        if (i >= right.length) { // right runs out of items before left = false
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
            if ((subVal !== undefined) && (subVal === true)) {
                return true;
            } else if ((subVal !== undefined) && (subVal === false)) {
                return false;
            }
        } else if (typeof left[i] === "object") { //make an array of the right number
            let subVal = checkPackets(left[i], [right[i]]);
            if ((subVal !== undefined) && (subVal === true)) {
                return true;
            } else if ((subVal !== undefined) && (subVal === false)) {
                return false;
            }
        } else if (typeof right[i] === "object") { //make an array of the left number
            let subVal = checkPackets([left[i]], right[i]);
            if ((subVal !== undefined) && (subVal === true)) {
                return true;
            } else if ((subVal !== undefined) && (subVal === false)) {
                return false;
            }
        }
    }
}
let indicies = [];
let sum = 0;
for (let i = 0; i < input.length; i++) {
    // This next line of code is an atrocity
    let [left, right] = input[i].split("\n").map(value => eval(value));
    let good = checkPackets(left, right);
    if (good || good === undefined) {
        indicies.push(i);
        sum += i + 1;
    }
}

console.log("Sum:", sum);