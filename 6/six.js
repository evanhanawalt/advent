const fs = require("fs");
const input = fs.readFileSync('input.txt', 'utf8');
let lastThree = [];
let result = 0;
for (let i = 0; i < input.length; i++) {
    if (lastThree.length < 13) {
        lastThree.push(input[i]);
    } else  {
        let set = new Set(lastThree);
        if(set.has(input[i]) || set.size !== 13){
            lastThree.shift()
            lastThree.push(input[i]);
        } else {
            result = i+1;
            break;
        }
    }
}
console.log("result",result);