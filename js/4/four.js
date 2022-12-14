const fs = require("fs");
const data = fs.readFileSync("input.txt", 'utf8').split('\n');
const mappedData = data.map(value => {
    const pair =value.split(',');
    const left = pair[0];
    const right = pair[1];
    return{
        left:{
            min:parseInt(left.split('-')[0]),
            max:parseInt(left.split('-')[1])
        },
        right: {
            min:parseInt(right.split('-')[0]),
            max:parseInt(right.split('-')[1])
        }
    };
})
const rangeSize = (side) => {
    return side.max - side.min;
}
let count = 0;
for (let element of mappedData) {
    const left = element.left;
    const right = element.right;
    for (let i = left.min; i <= left.max ; i++) {
        if (i >= right.min && i <= right.max) {
            count++;
            break;
        }
    }
}

console.log(count);