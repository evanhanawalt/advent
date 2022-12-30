const fs = require("fs");
const lines = fs.readFileSync("input.txt", "utf-8").split("\n");
const UP = "^";
const DOWN = "v";
const LEFT = "<";
const RIGHT = ">";
const directions = [UP, DOWN, LEFT, RIGHT];
const numRows = lines.length;
const numCols = lines[0].length;
const lastRowWall = numRows - 1;
const lastColWall = numCols - 1;

const getLCM = (num1, num2) => {
  let min = num1 > num2 ? num1 : num2;

  while (true) {
    if (min % num1 == 0 && min % num2 == 0) {
      break;
    }
    min++;
  }
  return min;
};

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
  arr.forEach((element) => {
    set.add(`${element.row},${element.col}`);
  });
  return set;
};

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

const sortNodes = (a, b) => a.time - b.time;

const findTimeToGoal = (start, end) => {
  const unvisited = [start];
  const visited = new Set();

  while (unvisited.length > 0) {
    unvisited.sort(sortNodes);
    let current = unvisited.shift();
    let nextMoves = [
      { row: current.row + 1, col: current.col, time: current.time + 1 },
      { row: current.row - 1, col: current.col, time: current.time + 1 },
      { row: current.row, col: current.col + 1, time: current.time + 1 },
      { row: current.row, col: current.col - 1, time: current.time + 1 },
      { row: current.row, col: current.col, time: current.time + 1 },
    ];
    for (let move of nextMoves) {
      // check for end state
      if (move.col === end.col && move.row === end.row) {
        return move;
      }
      // node can't be out of bounds, blocked or visited
      // don't forget starting position.....
      const outOfBounds =
        !(move.row === start.row && move.col === start.col) &&
        (move.col <= 0 ||
          move.col >= lastColWall ||
          move.row <= 0 ||
          move.row >= lastRowWall);
      if (
        outOfBounds ||
        allBlizzardPositions[move.time % loopLength].has(
          `${move.row},${move.col}`
        ) ||
        visited.has(`${move.row},${move.col},${move.time % loopLength}`)
      ) {
        continue;
      }

      visited.add(`${move.row},${move.col},${move.time % loopLength}`);
      unvisited.push(move);
    }
  }
};

const part1 = () => {
  const start = { row: 0, col: 1, time: 0 };
  const end = { row: 36, col: 100 };
  const finalState = findTimeToGoal(start, end);
  console.log("Part1:", finalState);
  return finalState;
};

part1();

const part2 = () => {
  const start = { row: 0, col: 1, time: 0 };
  const end = { row: 36, col: 100 };
  const trip2Start = findTimeToGoal(start, end);
  const trip3Start = findTimeToGoal(trip2Start, start);
  const finalState = findTimeToGoal(trip3Start, end);
  console.log("Part2: ", finalState);
};
part2();
