const fs = require("fs");
// This code is ugly because I didn't think about doing a search from the 
// final node to the nearest 'a' node for a long time, and I refuse to clean it up
const aList = [];
let finalNode = null
const characterGrid = fs.readFileSync("input.txt", "utf8").split("\n").map((line, rowIndex) => line.split("").map((char, colIndex) => {
    let node = {
        row: rowIndex,
        col: colIndex,
        value: char,
        height: char.charCodeAt(0),
        visited: false,
        distance: Infinity
    };

    if (char === "a") {
        aList.push(node);
    } else if (char === "S") {
        node.height = "a".charCodeAt(0);
        aList.push(node);
    } else if (char === "E") {
        node.height = "z".charCodeAt(0);
        finalNode = node;
    }
    return node;
}));
// map neighbors
let nodeList = [];
characterGrid.forEach(line => line.forEach(node => {
    const { row, col } = node;
    let potentialNeighbors = [
        { row: row + 1, col: col },
        { row: row - 1, col: col },
        { row: row, col: col + 1 },
        { row: row, col: col - 1 }
    ]
    node.neighbors = potentialNeighbors.reduce((accum, x) => {
        const isWithinGrid = (x.row >= 0 && x.row < characterGrid.length) && (x.col >= 0 && x.col < characterGrid[x.row].length);
        if (isWithinGrid) {
            // if destination at most 1 higher than current position, neighbor is allowed 
            const isAcceptableHeight = (node.height - characterGrid[x.row][x.col].height) <= 1;
            if (isAcceptableHeight) {
                accum.push(characterGrid[x.row][x.col]);
            }
        }
        return accum;
    }, []);
    nodeList.push(node);
}));
let shortestDistance = Infinity;

// sort node so last element is the lowest distance
let sortNodesList = (a, b) => {
    if (a.distance > b.distance) {
        return -1;
    } else if (a.distance < b.distance) {
        return 1;
    } else {
        return 0;
    }
}


let list = nodeList.map(node => {
    if (node.col === finalNode.col && node.row === finalNode.row) {
        node.distance = 0;
    } else {
        node.distance = Infinity;
    }
    node.visited = false;
    return node;
});
let finalValue = 123123123;
while (list.length > 0) {
    list.sort(sortNodesList);
    let currentNode = list.pop();
    let newDistance = currentNode.distance + 1;

    currentNode.neighbors.forEach(element => {
        if (!element.visited) {
            if (element.distance > newDistance) {
                element.distance = newDistance;
            }
        }
    });
    currentNode.visited = true;
    if (currentNode.value === "a") {
        finalValue = currentNode.distance;
        break;
    }
}


console.log(finalValue);