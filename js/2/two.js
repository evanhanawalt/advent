
const fs = require("fs");
const input = fs.readFileSync('./input.txt', 'utf8');
const lines = input.split('\n');
//A/X Rock
//B/Y Paper
//C/Z Scissors
const outcomeMap= new Map([
    ["A X", 3],
    ["A Y", 1],
    ["A Z", 2],
    ["B X", 1],
    ["B Y", 2],
    ["B Z", 3],
    ["C X", 2],
    ["C Y", 3],
    ["C Z", 1] 
]);
const choiceMap = new Map([
    ['X', 0],
    ['Y', 3],
    ['Z', 6]
]);
const score = lines.reduce((accumulator, currentValue) => accumulator += outcomeMap.get(currentValue) + choiceMap.get(currentValue[2]), 0);

console.log(score);