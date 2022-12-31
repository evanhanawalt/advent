class Node {
  constructor(key, value, parent) {
    this.key = key;
    this.value = value;
    this.children = [];
    this.parent = parent;
    this.totalValue = 0;
  }
  addChild(child) {
    this.children.push(child);
  }
  setTotalValue(totalValue) {
    this.totalValue = totalValue;
  }
}
const fs = require("fs");
const input = fs.readFileSync("input.txt", "utf8");
let root = null;
let lastNode = null;
let testCount = 0;
input.split("\n").forEach((line, index, array) => {
  if (line.includes("$ ls")) {
    const key = array[index - 1].split(" ")[2];
    let value = 0;
    let children = [];
    let parent = lastNode;
    for (let pointer = index + 1; pointer < array.length; pointer++) {
      if (array[pointer].startsWith("$")) {
        break;
      } else if (array[pointer].split(" ")[0] != "dir") {
        value += parseInt(array[pointer].split(" ")[0]);
      }
    }

    let currentNode = new Node(key, value, parent);
    if (parent != null) {
      parent.addChild(currentNode);
    } else {
      root = currentNode;
    }
    lastNode = currentNode;
  } else if (line.includes("cd ..")) {
    lastNode = lastNode.parent;
  }
});

function getNodeVal(rootNode) {
  let value = 0;
  if (rootNode.value === 56512) {
    console.log(rootNode);
  }

  if (rootNode.children.length > 0) {
    for (const element of rootNode.children) {
      value += getNodeVal(element);
    }
  }
  value += rootNode.value;

  rootNode.setTotalValue(value);
  return value;
}

const totalDiskSize = 70000000;
const desiredFreeSpace = 30000000;
const rootSize = getNodeVal(root);
console.log(rootSize);
const needToFree = desiredFreeSpace - (totalDiskSize - rootSize);
let currentDirSize = 999999999999999;
function findSmallestToDelete(rootNode) {
  if (rootNode.children.length > 0) {
    for (const element of rootNode.children) {
      findSmallestToDelete(element);
    }
  }
  if (
    rootNode.totalValue >= needToFree &&
    currentDirSize > rootNode.totalValue
  ) {
    currentDirSize = rootNode.totalValue;
  }
}

findSmallestToDelete(root);
console.log(currentDirSize);
