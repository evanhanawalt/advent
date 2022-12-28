const fs = require("fs");
const data = fs.readFileSync("input.txt", "utf-8");
const inputs = data.split("\n");

const part1 = () => {
    const nums = inputs.map(v => parseInt(v));
    const list = inputs.map((v, i) => ({ num: parseInt(v), id: i }));
    for (let i = 0; i < nums.length; i++) {
        const id = list.findIndex(x => x.id === i);
        list.splice(id, 1);
        list.splice((nums[i] + id) % list.length, 0, { num: nums[i], id: i });
    }
    const idZero = list.findIndex(x => x.num === 0);
    console.log(`Part1: ${[1000, 2000, 3000].reduce((prev, curr) => prev + list[(curr + idZero) % list.length].num, 0)}`);
}


part1();


const part2 = () => {
    const times = 10;
    const decKey = 811589153;
    const nums = inputs.map(v => v * decKey);
    const list = inputs.map((v, i) => ({ num: v * decKey, id: i }));

    for (let j = 0; j < times; j++) {
        for (let i = 0; i < nums.length; i++) {
            const id = list.findIndex(x => x.id === i);
            list.splice(id, 1);
            list.splice((nums[i] + id) % list.length, 0, { num: nums[i], id: i });
        }
    }

    const idZero = list.findIndex(x => x.num === 0);
    console.log(`Part2: ${[1000, 2000, 3000].reduce((prev, curr) => prev + list[(curr + idZero) % list.length].num, 0)}`);
};

part2();