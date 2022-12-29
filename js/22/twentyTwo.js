const fs = require("fs");
const [top, bottom] = fs.readFileSync("input.txt", "utf-8").split("\n\n");
const coords = new Map();
const rowMaxes = new Map();
const rowMins = new Map();
const colMaxes = new Map();
const colMins = new Map();
const positionTurns = new Map();
positionTurns.set(">L", "^");
positionTurns.set(">R", "v");
positionTurns.set("<L", "v");
positionTurns.set("<R", "^");
positionTurns.set("^L", "<");
positionTurns.set("^R", ">");
positionTurns.set("vL", ">");
positionTurns.set("vR", "<");
top.split("\n").forEach((line, rowIndex) => {
  rowMaxes.set(rowIndex, -1);
  rowMins.set(rowIndex, Infinity);
  line.split("").forEach((char, colIndex) => {
    // if this isn't blank
    if (char !== " ") {
      if (rowMaxes.get(rowIndex) < colIndex) {
        rowMaxes.set(rowIndex, colIndex);
      }
      if (rowMins.get(rowIndex) > colIndex) {
        rowMins.set(rowIndex, colIndex);
      }

      if (
        colMaxes.get(colIndex) === undefined ||
        colMaxes.get(colIndex) < rowIndex
      ) {
        colMaxes.set(colIndex, rowIndex);
      }
      if (
        colMins.get(colIndex) === undefined ||
        colMins.get(colIndex) > rowIndex
      ) {
        colMins.set(colIndex, rowIndex);
      }
      coords.set(`${rowIndex},${colIndex}`, char);
    }
  });
});

const part1 = () => {
  const turns = bottom.split(/\d*/).filter((char) => char !== "");
  const steps = bottom
    .split(/[LR]/)
    .filter((char) => char !== "")
    .map(Number);
  const nextPosition = (row, col, facing) => {
    let nextRow = null;
    let nextCol = null;
    if (facing === ">") {
      nextRow = row;
      if (coords.has(`${row},${col + 1}`)) {
        nextCol = col + 1;
      } else {
        nextCol = rowMins.get(row);
      }
    } else if (facing === "<") {
      nextRow = row;
      if (coords.has(`${row},${col - 1}`)) {
        nextCol = col - 1;
      } else {
        nextCol = rowMaxes.get(row);
      }
    } else if (facing === "v") {
      nextCol = col;
      if (coords.has(`${row + 1},${col}`)) {
        nextRow = row + 1;
      } else {
        nextRow = colMins.get(col);
      }
    } else if (facing === "^") {
      nextCol = col;
      if (coords.has(`${row - 1},${col}`)) {
        nextRow = row - 1;
      } else {
        nextRow = colMaxes.get(col);
      }
    }
    if (nextCol === null || nextRow === null) {
      throw new Error("This Shouldn't Happen");
    }

    return { row: nextRow, col: nextCol };
  };
  const startRow = 0;
  const startCol = rowMins.get(0);
  const position = { row: startRow, col: startCol, facing: ">" };
  const startingPosition = { row: startRow, col: startCol };

  while (steps.length > 0) {
    let currentSteps = steps.shift();
    while (currentSteps > 0) {
      const nextPos = nextPosition(position.row, position.col, position.facing);
      if (coords.get(`${nextPos.row},${nextPos.col}`) === "#") {
        break;
      }
      position.row = nextPos.row;
      position.col = nextPos.col;
      currentSteps--;
    }

    let turn = turns.shift();
    let nextFacing = positionTurns.get(`${position.facing}${turn}`);
    if (!nextFacing) {
      continue;
    }
    position.facing = nextFacing;
  }

  let facingVal = 0;
  if (position.facing === "v") facingVal = 1;
  if (position.facing === "<") facingVal = 2;
  if (position.facing === "^") facingVal = 3;

  const sum = (position.row + 1) * 1000 + (position.col + 1) * 4 + facingVal;
  console.log("Part1:", sum);
};

//part1();

const part2 = () => {
  const turns = bottom.split(/\d*/).filter((char) => char !== "");
  const steps = bottom
    .split(/[LR]/)
    .filter((char) => char !== "")
    .map(Number);

  const performReposition = (nextRow, nextCol, facing) => {
    ////stupid mapping.....
    return { row: 0, col: 0, facing: "" };
  };

  const nextPosition = (row, col, facing) => {
    let nextRow = null;
    let nextCol = null;
    if (facing === ">") {
      nextRow = row;
      if (coords.has(`${row},${col + 1}`)) {
        nextCol = col + 1;
      } else {
        nextCol = rowMins.get(row);
      }
    } else if (facing === "<") {
      nextRow = row;
      if (coords.has(`${row},${col - 1}`)) {
        nextCol = col - 1;
      } else {
        nextCol = rowMaxes.get(row);
      }
    } else if (facing === "v") {
      nextCol = col;
      if (coords.has(`${row + 1},${col}`)) {
        nextRow = row + 1;
      } else {
        nextRow = colMins.get(col);
      }
    } else if (facing === "^") {
      nextCol = col;
      if (coords.has(`${row - 1},${col}`)) {
        nextRow = row - 1;
      } else {
        nextRow = colMaxes.get(col);
      }
    }
    if (nextCol === null || nextRow === null) {
      throw new Error("This Shouldn't Happen");
    }

    return { row: nextRow, col: nextCol };
  };
  const startRow = 0;
  const startCol = rowMins.get(0);
  const position = { row: startRow, col: startCol, facing: ">" };
  const startingPosition = { row: startRow, col: startCol };

  while (steps.length > 0) {
    let currentSteps = steps.shift();
    while (currentSteps > 0) {
      const nextPos = nextPosition(position.row, position.col, position.facing);
      if (coords.get(`${nextPos.row},${nextPos.col}`) === "#") {
        break;
      }
      position.row = nextPos.row;
      position.col = nextPos.col;
      currentSteps--;
    }

    let turn = turns.shift();
    let nextFacing = positionTurns.get(`${position.facing}${turn}`);
    if (!nextFacing) {
      continue;
    }
    position.facing = nextFacing;
  }

  let facingVal = 0;
  if (position.facing === "v") facingVal = 1;
  if (position.facing === "<") facingVal = 2;
  if (position.facing === "^") facingVal = 3;

  const sum = (position.row + 1) * 1000 + (position.col + 1) * 4 + facingVal;
  console.log("Part2:", sum);
};

part2();
