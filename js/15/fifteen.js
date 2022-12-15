const fs = require("fs");
const distance = (coord1, coord2) => {
    return Math.abs(coord1.x - coord2.x) + Math.abs(coord1.y - coord2.y);
}
const pairs = fs.readFileSync("input.txt", "utf8").split("\n").map(line => {
    let coordPair = line
        .substring("Sensor at".length)
        .split(": closest beacon is at")
        .map(coordString => {
            let coords = coordString.split(",").map(value => {
                return parseInt(value.trim().substring(2));
            })
            return { x: coords[0], y: coords[1] };
        });
    return {
        sensor: coordPair[0],
        beacon: coordPair[1],
        manhattanD: distance(coordPair[0], coordPair[1])
    };
});
const maxCoordinate = 4000000;
// Check 1 column at a time
let yResult = false;
for (let x = 0; x <= maxCoordinate; x++) {
    const columnRanges = [];
    for (const { sensor, manhattanD } of pairs) {
        const columnToSensorD = distance(sensor, { x, y: sensor.y });
        if (columnToSensorD <= manhattanD) { //col is seen by the sensor
            const intersectionD = manhattanD - columnToSensorD;
            columnRanges.push({
                min: sensor.y - intersectionD,
                max: sensor.y + intersectionD
            });
        }
    }
    columnRanges.sort((a, b) => a.min - b.min); // order by smallest min
    const mergedRange = columnRanges.shift();
    while (columnRanges.length > 0) {
        const currentRange = columnRanges.shift();
        if (mergedRange.max >= currentRange.min) { // merged max overlaps!
            if (currentRange.max > mergedRange.max) { // current max is bigger!
                mergedRange.max = currentRange.max;
            }
        } else { // no overlap, and all mins left are bigger 
            yResult = mergedRange.max + 1;
            break;
        }
    }
    if (yResult) {
        console.log(`result: ${x}, ${yResult}`);
        console.log("tuning frequency: ", (4000000 * x) + yResult);
        break;
    }
}
