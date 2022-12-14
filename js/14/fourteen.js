const fs = require("fs");
const blockedTiles = new Set();
const tileToString = (tile) => `${tile.x},${tile.y}`;
let lowestPoint = 0;
fs.readFileSync("input.txt", "utf8").split("\n").forEach(line => {
    let verticies = line.split('->').map(vertex => {
        const [x, y] = vertex.split(',');
        return { x: parseInt(x), y: parseInt(y) };
    })
    // block each point between verticies
    for (let i = 0; i < verticies.length - 1; i++) {
        let start = verticies[i];
        let end = verticies[i + 1];

        // get the lowest point as well
        if (Math.max(start.y, end.y) > lowestPoint) {
            lowestPoint = Math.max(start.y, end.y);
        }

        // all lines are either horizontal or vertical
        // block every tile between the 2 verticies
        if (start.x === end.x) { // horizontal
            let increment = start.y < end.y ? 1 : -1;
            let x = start.x;
            for (let y = start.y; y != end.y; y += increment) {
                blockedTiles.add(tileToString({ x, y }));
            }
        } else if (start.y === end.y) {
            let increment = start.x < end.x ? 1 : -1;
            let y = start.y;
            for (let x = start.x; x != end.x; x += increment) {
                blockedTiles.add(tileToString({ x, y }));
            }
        }
        // dont forget the end block, this took me an hour to find
        blockedTiles.add(tileToString(end));

    }
})
let floorY = lowestPoint + 2;
let fallingIntoTheAbyss = false;
let sandCount = 0;

while (!fallingIntoTheAbyss) {

    let sand = { x: 500, y: 0 };
    while (!fallingIntoTheAbyss) {

        // stop the sand at the floor
        if (sand.y === floorY - 1) {
            sandCount++;
            blockedTiles.add(`${sand.x},${sand.y}`);
            break;
        } else if (!blockedTiles.has(`${sand.x},${sand.y + 1}`)) { // straight down is open
            sand.y++;
        } else if (!blockedTiles.has(`${sand.x - 1},${sand.y + 1}`)) { // down left is open
            sand.x--;
            sand.y++;
        } else if (!blockedTiles.has(`${sand.x + 1},${sand.y + 1}`)) { // down right is open
            sand.x++;
            sand.y++;
        } else { // nothing open, stop
            sandCount++;
            blockedTiles.add(`${sand.x},${sand.y}`);
            if (sand.y === 0) {
                fallingIntoTheAbyss = true;
            }
            break;
        }

    }
}

console.log("# of settled sand grains:", sandCount);