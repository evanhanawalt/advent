const fs = require("fs");
const lines = fs.readFileSync("input.txt", "utf-8").split("\n");
const initialPositions = new Set();
lines.forEach((line, rowIndex) => {
    line.split("").forEach((char, colIndex) => {
        if (char === "#") {
            initialPositions.add(`${rowIndex},${colIndex}`);
        }
    });
});

const getProposedMove = (row, col, firstDirection, takenPositions) => {
    const N = takenPositions.has(`${row - 1},${col}`);
    const NE = takenPositions.has(`${row - 1},${col + 1}`);
    const NW = takenPositions.has(`${row - 1},${col - 1}`);
    const S = takenPositions.has(`${row + 1},${col}`);
    const SE = takenPositions.has(`${row + 1},${col + 1}`);
    const SW = takenPositions.has(`${row + 1},${col - 1}`);
    const E = takenPositions.has(`${row},${col + 1}`);
    const W = takenPositions.has(`${row},${col - 1}`);
    const hasNeighbors = N || NE || NW || S || SE || SW || E || W;
    // no neightbors, no move
    if (!hasNeighbors) {
        return { row, col };
    }

    for (let j = 0; j < 4; j++) {
        const direction = (firstDirection + j) % 4;
        if (direction === 0 && !(N || NE || NW)) {//N
            return { row: row - 1, col };
        } else if (direction === 1 && !(S || SE || SW)) {//S
            return { row: row + 1, col };
        } else if (direction === 2 && !(W || NW || SW)) {//W
            return { row, col: col - 1 };
        } else if (direction === 3 && !(E || NE || SE)) {//E
            return { row, col: col + 1 };
        }
    }
    // no good move found
    return { row, col };
}

const part1 = () => {
    let elvesPos = initialPositions;
    for (let i = 0; i < 10; i++) {
        const proposedMoves = new Map();
        const proposedDestinations = new Set();
        const doubledDestinations = new Set();
        const currentFirstDir = i % 4;

        elvesPos.forEach((position) => {
            const [row, col] = position.split(",").map(Number);
            const { row: propRow, col: propCol } = getProposedMove(row, col, currentFirstDir, elvesPos);
            const destinationKey = `${propRow},${propCol}`;
            proposedMoves.set(position, destinationKey);
            if (proposedDestinations.has(destinationKey)) {
                doubledDestinations.add(destinationKey);
            } else {
                proposedDestinations.add(destinationKey);
            }
        });
        const newPositions = new Set();
        proposedMoves.forEach((destination, position) => {

            // if destination is doubled, don't move
            if (doubledDestinations.has(destination)) {
                newPositions.add(position);
            } else {
                newPositions.add(destination);
            }
        });
        if (elvesPos.size !== newPositions.size) {
            throw new Error("This shouldn't happen");
        }
        elvesPos = newPositions;
    }
    // find the smallest rectangle containing the elves position
    let rowMin = 0;
    let rowMax = 0;
    let colMin = 0;
    let colMax = 0;
    elvesPos.forEach(position => {
        const [row, col] = position.split(",").map(Number);
        if (row > rowMax) rowMax = row;
        if (row < rowMin) rowMin = row;
        if (col > colMax) colMax = col;
        if (col < colMin) colMin = col;
    });

    const length = 1 + colMax - colMin;
    const height = 1 + rowMax - rowMin;
    const totalArea = length * height;
    console.log(`Part1: ${totalArea - elvesPos.size}`);

}

part1();

const part2 = () => {

    let elvesPos = initialPositions;
    for (let i = 0; i != -10; i++) {
        const proposedMoves = new Map();
        const proposedDestinations = new Set();
        const doubledDestinations = new Set();
        const currentFirstDir = i % 4;

        elvesPos.forEach((position) => {
            const [row, col] = position.split(",").map(Number);
            const { row: propRow, col: propCol } = getProposedMove(row, col, currentFirstDir, elvesPos);
            const destinationKey = `${propRow},${propCol}`;
            proposedMoves.set(position, destinationKey);
            if (proposedDestinations.has(destinationKey)) {
                doubledDestinations.add(destinationKey);
            } else {
                proposedDestinations.add(destinationKey);
            }
        });
        const newPositions = new Set();
        proposedMoves.forEach((destination, position) => {

            // if destination is doubled, don't move
            if (doubledDestinations.has(destination)) {
                newPositions.add(position);
            } else {
                newPositions.add(destination);
            }
        });
        if (elvesPos.size !== newPositions.size) {
            throw new Error("This shouldn't happen");
        }

        // check if the new positions are exactly the same as the old positions
        const nothingChanged = Array.from(newPositions).every((newValue) => elvesPos.has(newValue));
        if (nothingChanged) {
            console.log(`Part2: ${i + 1}`);
            break;
        }
        elvesPos = newPositions;
    }

}

part2();