const fs = require("fs");
const { listenerCount } = require("process");
const entries = fs.readFileSync("input.txt", "utf8").split("\n\n");
let data = []
// take in input
for (let entry of entries) {
    let monkey = {
        inspectionCount: 0
    };
    entry.split('\n').forEach(line => {
        if (line.includes("Starting items:")) {
            monkey.itemList = line.split(':')[1].split(',').map(value => parseInt(value));
        } else if (line.includes("Operation:")) {
            monkey.operation = `Math.floor((${line.split("new = ")[1]}) / 3)`
        } else if (line.includes("Test:")) {
            monkey.divisor = parseInt(line.split("divisible by")[1]);
        } else if (line.includes("If true:")) {
            monkey.ifTrue = parseInt(line.split("monkey")[1]);
        } else if (line.includes("If false:")) {
            monkey.ifFalse = parseInt(line.split("monkey")[1]);
        }
    })
    data.push(monkey);

}

const doRound = (index, monkeyList) => {
    monkeyList[index].itemList.forEach(value => {
        let old = value;
        let newVal = eval(monkeyList[index].operation);
        if (newVal % monkeyList[index].divisor === 0) {
            monkeyList[monkeyList[index].ifTrue].itemList.push(newVal);
        } else {
            monkeyList[monkeyList[index].ifFalse].itemList.push(newVal);
        }
        monkeyList[index].inspectionCount++;
    });
    monkeyList[index].itemList = [];
}

for (let i = 0; i < 20; i++) {
    for (let monkeyIndex = 0; monkeyIndex < data.length; monkeyIndex++) {
        doRound(monkeyIndex, data);
    }
}

let sortedValues = [];
data.forEach((element, index) => {
    sortedValues.push(element.inspectionCount);
});

sortedValues.sort((a, b) => b - a);
console.log(sortedValues[0] * sortedValues[1]);