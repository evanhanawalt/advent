const fs = require("fs");

const lines = fs.readFileSync("input.txt", "utf-8").split("\n");
const canBuildBot = (
  { oreCost, clayCost, obsidianCost },
  { oreCount, clayCount, obsidianCount }
) => {
  // can't build if there is a cost, and the cost is greater than what we have
  if (oreCost && oreCost > oreCount) {
    return false;
  } else if (clayCost && clayCost > clayCount) {
    return false;
  } else if (obsidianCost && obsidianCost > obsidianCount) {
    return false;
  }
  return true;
};
const bluePrints = lines.map((line) => {
  const split = line.split(" ");
  const bluePrint = {
    oreBot: { oreCost: parseInt(split[6]) },
    clayBot: { oreCost: parseInt(split[12]) },
    obsidianBot: {
      oreCost: parseInt(split[18]),
      clayCost: parseInt(split[21]),
    },
    geodeBot: {
      oreCost: parseInt(split[27]),
      obsidianCost: parseInt(split[30]),
    },
  };

  return bluePrint;
});
let result = 0;
