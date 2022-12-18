const fs = require("fs");
const drops = fs
  .readFileSync("input.txt", "utf8")
  .split("\n")
  .map((line) => {
    let split = line.split(",");
    return {
      x: parseInt(split[0]),
      y: parseInt(split[1]),
      z: parseInt(split[2]),
    };
  });

const part1 = () => {
  const coversOneSide = (mainDrop, secondDrop) => {
    const diffX = Math.abs(mainDrop.x - secondDrop.x);
    const diffY = Math.abs(mainDrop.y - secondDrop.y);
    const diffZ = Math.abs(mainDrop.z - secondDrop.z);
    if (diffX === 0 && diffY === 0) {
      return diffZ === 1;
    }
    if (diffX === 0 && diffZ === 0) {
      return diffY === 1;
    }
    if (diffY === 0 && diffZ === 0) {
      return diffX === 1;
    }
  };
  let totalSurfaceArea = 0;

  drops.forEach((drop) => {
    let count = drops.reduce((accum, secondDrop) => {
      if (coversOneSide(drop, secondDrop)) {
        return accum - 1;
      }
      return accum;
    }, 6);

    totalSurfaceArea += count;
  });

  console.log("Total Surface Area:", totalSurfaceArea);
};

const part2 = () => {
  console.log("Part 2");
  const visited = new Set();
  const lavaPositions = new Set();
  let min = 1000;
  let max = 0;
  for (const { x, y, z } of drops) {
    lavaPositions.add(`${x},${y},${z}`);
    min = Math.min(min, x, y, z);
    max = Math.max(max, x, y, z);
  }

  // checks if the input is adjacent to the lava
  const countAdjacentSurface = ({ x, y, z }) => {
    let count = 0;
    if (lavaPositions.has(`${x + 1},${y},${z}`)) count++;
    if (lavaPositions.has(`${x - 1},${y},${z}`)) count++;
    if (lavaPositions.has(`${x},${y + 1},${z}`)) count++;
    if (lavaPositions.has(`${x},${y - 1},${z}`)) count++;
    if (lavaPositions.has(`${x},${y},${z + 1}`)) count++;
    if (lavaPositions.has(`${x},${y},${z - 1}`)) count++;

    return count;
  };

  const searchMin = min - 1;
  const searchMax = max + 1;
  const searchQueue = [{ x: 0, y: 0, z: 0 }];
  let externalSurfaceArea = 0;
  while (searchQueue.length > 0) {
    let current = searchQueue.shift();
    let { x, y, z } = current;
    let currentKey = `${x},${y},${z}`;
    // Skip visited nodes, lava nodes, or nodes beyond the min / max
    if (visited.has(currentKey)) continue;
    if (lavaPositions.has(currentKey)) continue;
    if (x < searchMin || y < searchMin || z < searchMin) continue;
    if (x > searchMax || y > searchMax || z > searchMax) continue;

    // this is an in-bounds water coordinate
    visited.add(currentKey);
    externalSurfaceArea += countAdjacentSurface(current);

    searchQueue.push({ x: x + 1, y, z });
    searchQueue.push({ x: x - 1, y, z });
    searchQueue.push({ x, y: y + 1, z });
    searchQueue.push({ x, y: y - 1, z });
    searchQueue.push({ x, y, z: z + 1 });
    searchQueue.push({ x, y, z: z - 1 });
  }
  console.log("External Surface Area:", externalSurfaceArea);
};
//part1();
part2();
