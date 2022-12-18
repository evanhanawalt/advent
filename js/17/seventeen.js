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

const findLongestPattern = (arr) => {
  let longestPattern = [];
  let maxL = Math.floor(arr.length / 2);
  while (maxL > 1) {
    for (let i = 0; i + maxL < arr.length; i++) {
      let pattern = [];
      for (let j = 0; j < maxL; j++) {
        if (arr[i + j] === arr[i + j + maxL]) {
          pattern.push(arr[i + j]);
        } else {
          break;
        }
      }
      if (pattern.length === maxL) {
        return pattern;
      }
    }
    maxL--;
  }

  return longestPattern;
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
let patternChecker = [];
let lastBlocked = null;
for (let r = 0; r < 1000000000000; r++) {
  let currentRock = getInitialRock(r, highestRock);
  const rockType = r % 5;
  const pKey = makePatternKey(
    blockedCoordinates,
    rockType,
    inputIndex,
    highestRock
  );
  // if we've seen this pattern, skip the calculation
  if (patternMap.has(pKey)) {
    // set up data to look for a pattern, skip through patter at 6001
    if (r > 2000 && r < 6000) {
      patternChecker.push(pKey);
    } else if (r === 6001) {
      // we have data, find the biggest pattern in there
      const actualPattern = findLongestPattern(patternChecker);
      let numTimesToSimulate = Math.floor(
        (1000000000000 - r) / actualPattern.length
      );
      // when simulating the pattern, rock and inputIndex should remain the same
      let patternHeightDelta = actualPattern
        .map((key) => {
          return patternMap.get(key).heightDelta;
        })
        .reduce((accum, value) => accum + value, 0);

      let totalHeightAdded = numTimesToSimulate * patternHeightDelta;
      let totalRocksDropped = numTimesToSimulate * actualPattern.length;
      r += totalRocksDropped;
      highestRock += totalHeightAdded;
    }
    const { heightDelta, blocked, endIndex } = patternMap.get(pKey);
    inputIndex = endIndex;
    highestRock += heightDelta;
    lastBlocked = blocked;
    blockedCoordinates = blocked.map((node) => {
      return `${node.x},${node.y + highestRock}`;
    });
  } else {
    // we haven't seen this exact input, so do the simulation for it
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
        // this rock is done,
        let lastTop = highestRock;
        currentRock.forEach((coordinate) => {
          if (coordinate.y > highestRock) {
            highestRock = coordinate.y;
          }
          setBlockedCoord(coordinate);
        });
        let topDiff = highestRock - lastTop;
        let full = false;
        blockedCoordinates = blockedCoordinates.filter((val) => {
          let h = highestRock - parseInt(val.split(",")[1]);
          if (h >= 68) {
            full = true;
          }
          return h < 69;
        });

        let blocked = [];
        blockedCoordinates.forEach((val) => {
          let split = val.split(",");
          let x = parseInt(split[0]);
          let y = parseInt(split[1] - highestRock);
          blocked.push({ x, y });
        });

        if (full) {
          patternMap.set(pKey, {
            heightDelta: topDiff,
            blocked: blocked,
            endIndex: inputIndex,
          });
        }

        break;
      }
    }
  }
}
console.log(highestRock + 1);
