const fs = require("fs");
const readInput = () => {
    let list = [];
    try {
        const data = fs.readFileSync("./input.txt", 'utf8');
        let thing = [];
        data.split('\n\n').forEach((value) => {
            let subList = [];
            value.split('\n').forEach((value) => {
                subList.push(parseInt(value));
            });
            
            list.push(subList);
            
        });
    } catch (err) {
        console.error(err);
    }
    
    return list;
}


const elfWithTheMost = (listOfLists) => {
    let currentMax = [];
    for (const element of listOfLists) {
        let sum = element.reduce((accumumlator, currentValue) => accumumlator +currentValue);
        if (currentMax.length < 3){
            currentMax.push(sum);
        } else {
            const toBeat = Math.min(...currentMax);
            const indexToBeat = currentMax.indexOf(toBeat);
            if (sum > toBeat) {
                currentMax[indexToBeat] = sum;
            }
        }
    }
    return currentMax;
} 

const input = readInput();

console.log(elfWithTheMost(input).reduce((acc, current) => acc+ current));
