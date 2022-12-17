console.time("time");
const fs = require("fs");
const path = require("path");
const input = fs.readFileSync("input.txt", "utf-8").split("\n");
// make a map of nodes to lookup, both with and without flowrates
const nodes = input.map((line, index) => {
  let tokens = line.split(" ");
  return {
    index: index,
    name: tokens[1],
    flowRate: parseInt(tokens[4].substring(5, tokens[4].length - 1)),
    connections: tokens
      .slice(tokens.indexOf("to") + 2)
      .map((v) => v.substring(0, 2)),
    opened: false,
  };
});
const getNode = (n) => nodes.filter((node) => node.name == n)[0];
const distances = new Map();
const sorter = (a, b) => {
  if (a.distance > b.distance) {
    return -1;
  } else if (a.distance < b.distance) {
    return 1;
  } else {
    return 0;
  }
};
// compute node distances for the interesting nodes
const interestingNames = nodes
  .filter((node) => node.flowRate > 0 || node.name === "AA")
  .map((node) => node.name);
for (let interestingName of interestingNames) {
  const unvisited = Array.from(nodes);
  unvisited.forEach((el) => {
    el.distance = Infinity;
    if (el.name === interestingName) {
      el.distance = 0;
    }
  });

  const nodeMap = new Map();
  unvisited.forEach((element) => {
    nodeMap.set(element.name, element);
  });

  const visitedNames = new Set();
  let results = [];
  while (unvisited.length > 0) {
    unvisited.sort(sorter);
    let current = unvisited.pop();
    let neighbors = unvisited.filter((node) =>
      current.connections.includes(node.name)
    );
    neighbors.forEach((neighbor) => {
      if (!visitedNames.has(neighbor.name)) {
        neighbor.distance = current.distance + 1;
      }
    });
    visitedNames.add(current.name);
    if (interestingNames.includes(current.name)) {
      results.push(current);
    }
  }
  results.sort(sorter);
  distances.set(interestingName, new Map());
  for (let result of results) {
    distances.get(interestingName).set(result.name, result.distance);
  }
}

let timeRemaining = 30;
let position = "AA";
let result = 0;
let thingsToVisit = Array.from(nodes.filter((n) => n.flowRate > 0));
// console.log(thingsToVisit.length);

const root = {
  name: "AA",
  time: 26,
  flow: 0,
  visited: [],
};
const queue = [];
queue.push(root);
let paths = [];
while (queue.length > 0) {
  let c = queue.shift();
  const options = thingsToVisit.filter(
    (v) => !c.visited.includes(v.name) && v.flowRate > 0
  );
  for (let opt of options) {
    const steps = distances.get(c.name).get(opt.name) + 1;

    if (c.time - steps <= 0) {
      // if (c.flow > maxFlow) {
      //     maxFlow = c.flow;
      // }
      // instead of checking max, put the record into a list
      paths.push(c);
    } else {
      queue.push({
        name: opt.name,
        time: c.time - steps,
        flow: c.flow + opt.flowRate * (c.time - steps),
        visited: [...c.visited, opt.name],
      });
    }
  }
}
const flowSorter = (a, b) => {
  return b.flow - a.flow;
};
paths.sort(flowSorter);
console.log(paths[0]);
// with paths sorted, when our first human path has
// already been tried as an elephant path, we won't find a higher value

let maxFlow = 0;
let firstValidElephant = Infinity;
for (let i = 0; i < paths.length && i <= firstValidElephant; i++) {
  for (let e = i + 1; e < paths.length; e++) {
    if (paths[i].visited.every((valve) => !paths[e].visited.includes(valve))) {
      //these paths don't intersect
      if (firstValidElephant === Infinity) {
        firstValidElephant = e;
      }
      let newFlow = paths[i].flow + paths[e].flow;
      if (maxFlow < newFlow) {
        maxFlow = newFlow;
      }
      // the first time we find an elephant we should break, cause there are no larger values..
      break;
    }
  }
}

console.log(maxFlow);
