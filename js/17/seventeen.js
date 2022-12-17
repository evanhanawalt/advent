const fs = require("fs");
const { default: test } = require("node:test");
let inputs = fs.readFileSync("input.txt", "utf8").split("");
const getInitialRock = (index, highestRock) => {
  const rockType = index % 5;
  const yMin = highestRock + 4;
  if (rockType === 0) {
    return [
      { x: 2, y: yMin }, //     0123456
      { x: 3, y: yMin }, //     ..####
      { x: 4, y: yMin },
      { x: 5, y: yMin },
    ];
  } else if (rockType === 1) {
    return [
      { x: 3, y: yMin + 2 }, // 0123456
      { x: 2, y: yMin + 1 }, // ...#
      { x: 3, y: yMin + 1 }, // ..###
      { x: 4, y: yMin + 1 }, // ...#
      { x: 3, y: yMin },
    ];
  } else if (rockType === 2) {
    return [
      { x: 2, y: yMin }, //     0123456
      { x: 3, y: yMin }, //     ....#
      { x: 4, y: yMin }, //     ....#
      { x: 4, y: yMin + 1 }, // ..###
      { x: 4, y: yMin + 2 },
    ];
  } else if (rockType === 3) {
    return [
      { x: 2, y: yMin }, //     0123456
      { x: 2, y: yMin + 1 }, // ..#
      { x: 2, y: yMin + 2 }, // ..#
      { x: 2, y: yMin + 3 }, // ..#
    ]; //                       ..#
  } else if (rockType === 4) {
    return [
      { x: 2, y: yMin }, //     0123456
      { x: 3, y: yMin }, //     ..##
      { x: 2, y: yMin + 1 }, // ..##
      { x: 3, y: yMin + 1 },
    ];
  }
};
let blockedCoordinates = [];
const patternMap = new Map();
const coordKey = (x, y) => `${x},${y}`;
const setBlockedCoord = ({ x, y }) => {
  blockedCoordinates.push(coordKey(x, y));
};
const coordinateIsBlocked = ({ x, y }) => {
  // x,y >0, x >= 7 === blocked
  if (x < 0 || x > 6 || y < 0) {
    return true;
  }
  return blockedCoordinates.includes(coordKey(x, y));
};

const makePatternKey = (arr, rockType, currentInput, currHeight) => {
  const normalized = arr.map((str) => {
    let split = str.split(",");
    split[1] = currHeight - parseInt(split[1]);
    return split.join();
  });
  return `${normalized.join("|")},${rockType},${currentInput}`;
};
let highestRock = -1;
let inputIndex = 0;
// drop rocks
let patternStart = "";

for (let r = 0; r < 1000000000000; r++) {
  let currentRock = getInitialRock(r, highestRock);
  const rockType = r % 5;
  if (rockType === 0 && inputIndex === 0) {
    console.log(`R:${r}, i:${inputIndex}`);
  }
  const pKey = makePatternKey(
    blockedCoordinates,
    rockType,
    inputIndex,
    highestRock
  );
  // if we've seen this pattern, skip the calculation
  if (patternMap.has(pKey)) {
    const { heightDelta, blockThese, endIndex } = patternMap.get(pKey);
    blockThese
      .map((node) => {
        return { x: node.x, y: node.y + highestRock };
      })
      .forEach((node) => {
        setBlockedCoord(node);
      });
    blockedCoordinates = blockedCoordinates.filter((val) => {
      return highestRock - parseInt(val.split(",")[1]) < 69;
    });
    inputIndex = endIndex;
    highestRock += heightDelta;
  } else {
    while (true) {
      // do horizontal push

      const direction = inputs[inputIndex];
      inputIndex = inputIndex + 1;
      if (inputIndex === inputs.length) {
        inputIndex = 0;
      }

      const xDelta = direction === "<" ? -1 : 1;

      const canMoveHorizontal = !currentRock.some(({ x, y }) => {
        const next = {
          x: x + xDelta,
          y: y,
        };
        return coordinateIsBlocked(next);
      });

      if (canMoveHorizontal) {
        currentRock = currentRock.map(({ x, y }) => {
          return { x: x + xDelta, y: y };
        });
      }
      // do drop or stop rock from falling
      const canDrop = !currentRock.some(({ x, y }) => {
        const next = {
          x: x,
          y: y - 1,
        };
        return coordinateIsBlocked(next);
      });

      if (canDrop) {
        currentRock = currentRock.map(({ x, y }) => {
          return { x: x, y: y - 1 };
        });
      } else {
        let lastTop = highestRock;
        let blockThese = [];
        currentRock.forEach((coordinate) => {
          if (coordinate.y > highestRock) {
            highestRock = coordinate.y;
          }
          blockThese.push({ x: coordinate.x, y: coordinate.y - lastTop });
          setBlockedCoord(coordinate);
        });
        let topDiff = highestRock - lastTop;
        patternMap.set(pKey, {
          heightDelta: topDiff,
          blockThese: blockThese,
          endIndex: inputIndex,
        });
        console.log(`R:${r},PatternSize:${patternMap.size}`);
        blockedCoordinates = blockedCoordinates.filter((val) => {
          return highestRock - parseInt(val.split(",")[1]) < 69;
        });
        break;
      }
    }
  }
}
console.log(highestRock + 1);
