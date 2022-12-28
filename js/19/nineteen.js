const fs = require("fs");
const lines = fs.readFileSync("input.txt", "utf-8").split("\n");
const timeLimit = 32;
const wholeList = lines.map((line) => {
  const split = line.split(" ");
  return {
    oreCost: parseInt(split[6]),
    clayCost: parseInt(split[12]),
    obsidianCostOre: parseInt(split[18]),
    obsidianCostClay: parseInt(split[21]),
    geodeCostOre: parseInt(split[27]),
    geodeCostObsidian: parseInt(split[30]),
  };
});

const bluePrints = wholeList.slice(0, 3);
console.log(bluePrints.length);



class State {
  blueprint = {};
  constructor(blueprint) {
    this.blueprint = blueprint;
    this.ore = 0;
    this.clay = 0;
    this.obsidian = 0;
    this.geode = 0;

    this.oreRobots = 1;
    this.clayRobots = 0;
    this.obsidianRobots = 0;
    this.geodeRobots = 0;
    this.minute = 0;
  }

  collect() {
    this.minute++;
    this.ore += this.oreRobots;
    this.clay += this.clayRobots;
    this.obsidian += this.obsidianRobots;
    this.geode += this.geodeRobots;
    return this;
  }
  canBuildOrerobot() {
    return this.ore >= this.blueprint.oreCost;
  }

  canBuildClayrobot() {
    return this.ore >= this.blueprint.clayCost;
  }

  canBuildObsidianrobot() {
    return (
      this.ore >= this.blueprint.obsidianCostOre &&
      this.clay >= this.blueprint.obsidianCostClay
    );
  }

  canBuildGeoderobot() {
    return (
      this.ore >= this.blueprint.geodeCostOre &&
      this.obsidian >= this.blueprint.geodeCostObsidian
    );
  }

  buildOrerobot() {
    this.ore -= this.blueprint.oreCost;
    this.oreRobots++;

    this.ore--;

    return this;
  }

  buildClayrobot() {
    this.ore -= this.blueprint.clayCost;
    this.clayRobots++;

    this.clay--;

    return this;
  }

  buildObsidianrobot() {
    this.ore -= this.blueprint.obsidianCostOre;
    this.clay -= this.blueprint.obsidianCostClay;
    this.obsidianRobots++;

    this.obsidian--;
    return this;
  }

  buildGeoderobot() {
    this.ore -= this.blueprint.geodeCostOre;
    this.obsidian -= this.blueprint.geodeCostObsidian;
    this.geodeRobots++;

    this.geode--;

    return this;
  }

  clone() {
    const state = new State(this.blueprint);
    state.ore = this.ore;
    state.clay = this.clay;
    state.obsidian = this.obsidian;
    state.geode = this.geode;
    state.oreRobots = this.oreRobots;
    state.clayRobots = this.clayRobots;
    state.obsidianRobots = this.obsidianRobots;
    state.geodeRobots = this.geodeRobots;
    state.minute = this.minute;
    return state;
  }
}

function getFitness(factory) {
  const { geode } = factory;
  const { oreRobots, clayRobots, obsidianRobots, geodeRobots } = factory;
  const minute = factory.minute;
  const remaining = timeLimit - minute;

  //figure out how much future geode production we can get
  const futureGeodes = geode + remaining * geodeRobots;

  //compile that into a score
  return (
    futureGeodes * 10000000 +
    obsidianRobots * 10000 +
    clayRobots * 100 +
    oreRobots
  );
}
let results = [];


console.log(bluePrints)
for (const bluePrint of bluePrints) {
  let states = [new State(bluePrint)];
  let nextGen = [];

  for (let time = 0; time < timeLimit; time++) {
    for (const state of states) {
      //build geode robots
      if (state.canBuildGeoderobot()) {
        nextGen.push(state.clone().buildGeoderobot().collect());
      }
      //build obsidian robots
      if (state.canBuildObsidianrobot()) {
        nextGen.push(state.clone().buildObsidianrobot().collect());
      }
      //build clay robots
      if (state.canBuildClayrobot()) {
        nextGen.push(state.clone().buildClayrobot().collect());
      }
      //build ore robots
      if (state.canBuildOrerobot()) {
        nextGen.push(state.clone().buildOrerobot().collect());
      }
      //don't build any robots
      nextGen.push(state.clone().collect());
    }

    //  prune 
    states = nextGen.map((state) => {
      return { state: state, fitness: getFitness(state) };
    }).sort((a, b) => b.fitness - a.fitness)
      .map(state => state.state).slice(0, 100000);
    nextGen = [];
  }


  const best = states.sort((a, b) => {
    return b.geode - a.geode;
  })[0];

  results.push(best);

}

console.log(results.map(a => a.geode).reduce((accum, val) => { return accum * val; }, 1));



console.log(value)