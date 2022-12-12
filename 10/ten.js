const fs = require("fs");
const lines = fs.readFileSync("input.txt", "utf8").split("\n");

let currentCycle = 1;
let x = 1;
let cycles = [];
for (let line of lines) {
    console.log(line);
    if (line.includes("noop")) {
        cycles.push(x);
        currentCycle++;
    } else if (line.includes("addx")) {
        let valueString = line.split(" ")[1];
        let value = parseInt(valueString);
        cycles.push(x);
        cycles.push(x);
        currentCycle += 2;
        x = x + value;
    }
}

let output = "";
for (let i = 0; i < cycles.length; i++) {
    let center = cycles[i];
    let currentX = i % 40;
    let isVisible = Math.abs(center - currentX) <= 1;
    if (isVisible) {
        output += "#";
    } else {
        output += ".";
    }
    if ((i + 1) % 40 === 0) {
        output += "\n";
    }
}
console.log(output);