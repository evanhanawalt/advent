const fs = require("fs");
const lines = fs.readFileSync("input.txt", "utf-8").split("\n");
const UP = "^";
const DOWN = "v";
const LEFT = "<";
const RIGHT = ">";
const directions = [UP, DOWN, LEFT, RIGHT];
const start = { row: 0, col: 1 };
const end = { row: 36, col: 100 };
const numRows = lines.length;
const numCols = lines[0].length;
const lastRowWall = numRows - 1;
const lastColWall = numCols - 1;

const getLCM = (num1, num2) => {
    let min = (num1 > num2) ? num1 : num2;

    while (true) {
        if (min % num1 == 0 && min % num2 == 0) {
            break;
        }
        min++;
    }
    return min;
}

const loopLength = getLCM(numRows - 2, numCols - 2);
let blizzards = [];
lines.forEach((line, rowIndex) => {
    line.split("").forEach((char, colIndex) => {
        if (directions.includes(char)) {
            blizzards.push({ dir: char, row: rowIndex, col: colIndex });
        }
    });
});


// the positions of all blizzards loop 
// simulate all possible blizzard positions...

const getSetOfBlizzardPositions = (arr) => {
    const set = new Set();
    arr.forEach(element => {
        set.add(`${element.row},${element.col}`);
    });
    return set;
}

const allBlizzardPositions = [getSetOfBlizzardPositions(blizzards)];

while (allBlizzardPositions.length < loopLength) {
    const newBlizzards = blizzards.map((bliz) => {
        let { row, col, dir } = bliz;
        let next = null;
        if (dir === UP) {
            next = { row: row - 1, col: col, dir };
        } else if (dir === DOWN) {
            next = { row: row + 1, col: col, dir };
        } else if (dir === LEFT) {
            next = { row: row, col: col - 1, dir };
        } else if (dir === RIGHT) {
            next = { row: row, col: col + 1, dir };
        } else {
            throw new Error("This shouldn't happen");
        }

        if (next.row === 0) {
            next.row = lastRowWall - 1;
        } else if (next.row === lastRowWall) {
            next.row = 1;
        } else if (next.col === 0) {
            next.col = lastColWall - 1;
        } else if (next.col === lastColWall) {
            next.col = 1;
        }
        return next;
    });
    allBlizzardPositions.push(getSetOfBlizzardPositions(newBlizzards));
    blizzards = newBlizzards;
}
