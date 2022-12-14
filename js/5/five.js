const fs = require('fs');
const input = fs.readFileSync('input.txt', 'utf8');

const [stacks, instructions ]= input.split('\n\n');
const stackLines = stacks.split('\n');
const instructionLines = instructions.split('\n');
const lastLine = stackLines.pop();
let stackList = new Map();
lastLine.trim().split("   ").forEach(value => {
    stackList.set(parseInt(value), []);
})
stackLines.reverse().forEach(line => {
    for (let i = 0; i < line.length; i++) {
        let char = line[i]
        if (char !== ' ' && char !== '[' && char != ']') {
            let stackNumer = ((i - 1)/4) +1;
            stackList.get(stackNumer).push(char);
        }
    }
})

for (let line of instructionLines) {
    const [quantString, rest] = line.slice(4).split('from');
    const [fromString, toString] = rest.split('to')
    const quant = parseInt(quantString);
    const fromStack = stackList.get(parseInt(fromString));
    const toStack  = stackList.get(parseInt(toString));
    toStack.push( ...fromStack.splice((fromStack.length - quant)));
}

let output = "";
for (let i = 1; i <=9; i++){
    output = output + stackList.get(i).pop();
}

console.log(output);
