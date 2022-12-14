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
            monkey.operation = line.split("new = ")[1];
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
// modulo ensures that the value of each item doesn't get too big, 
// after doing (val % modulo), the value should retain its common 
// factors, and disivibility checks will remain the same 
const modulo = data.reduce((acc, monk) => {
    return acc * monk.divisor;
}, 1);

const doRound = (index, monkeyList) => {
    monkeyList[index].itemList.forEach(value => {
        let old = value;
        let newVal = eval(monkeyList[index].operation) % modulo;
        if (newVal % monkeyList[index].divisor === 0) {
            monkeyList[monkeyList[index].ifTrue].itemList.push(newVal);
        } else {
            monkeyList[monkeyList[index].ifFalse].itemList.push(newVal);
        }
        monkeyList[index].inspectionCount += 1;
    });
    monkeyList[index].itemList = [];
}

for (let i = 0; i < 10000; i++) {
    for (let monkeyIndex = 0; monkeyIndex < data.length; monkeyIndex++) {
        doRound(monkeyIndex, data);
    }
}

let sortedValues = [];
data.forEach((element, index) => {
    sortedValues.push(element.inspectionCount);
});

console.log(data);
sortedValues.sort((a, b) => {
    if (a > b) {
        return -1;
    } else if (a < b) {
        return 1;
    } else {
        return 0;
    }
});
console.log((sortedValues[0] * sortedValues[1]).toString());