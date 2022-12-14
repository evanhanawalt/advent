const fs = require("fs");
const input = fs.readFileSync("input.txt", "utf8");
const line = input.split("\n");
let knots = [
    { x: 0, y: 0 },
    { x: 0, y: 0 },
    { x: 0, y: 0 },
    { x: 0, y: 0 },
    { x: 0, y: 0 },
    { x: 0, y: 0 },
    { x: 0, y: 0 },
    { x: 0, y: 0 },
    { x: 0, y: 0 },
    { x: 0, y: 0 }
]
let tailPositionHistory = new Set();
tailPositionHistory.add(`0 0`);
line.forEach((line) => {

    const [direction, stepsString] = line.split(" ");
    const steps = parseInt(stepsString);
    for (let i = steps; i > 0; i--) {
        // move head 1 step at a time
        if (direction == 'L') {
            knots[0].x--;
        } else if (direction == 'R') {
            knots[0].x++;
        } else if (direction == 'U') {
            knots[0].y++;
        } else if (direction == 'D') {
            knots[0].y--;
        }
        for (let i = 1; i < knots.length; i++) {
            // move the following
            let diffX = knots[i - 1].x - knots[i].x;
            let diffY = knots[i - 1].y - knots[i].y;
            let absX = Math.abs(diffX);
            let absY = Math.abs(diffY);
            if (absY > 1 || (absY === 1 && (absX > 1))) {
                if (diffY > 0) {
                    knots[i].y++;
                } else {
                    knots[i].y--;
                }
            }

            if (absX > 1 || (absX === 1 && (absY > 1))) {
                if (diffX > 0) {
                    knots[i].x++;
                } else {
                    knots[i].x--;
                }
            }
        }
        tailPositionHistory.add(`${knots[knots.length - 1].x} ${knots[knots.length - 1].y}`);
    }

});

console.log(tailPositionHistory.size);