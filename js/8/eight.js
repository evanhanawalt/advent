const fs = require("fs");
const input = fs.readFileSync('input.txt', 'utf8');
const lines = input.split("\n");
const grid = lines.map(value => value.split("").map(char => parseInt(char)));
function checkIsVisible(row, col, grid) {
    let cur = grid[row][col];
    let checkL = 0;
    let checkR = 0;
    let checkT = 0;
    let checkB = 0;

    //check left
    for (let i = col -1; i >= 0; i--){
        checkL++;
        if (cur <= grid[row][i]){
            break;
        }
    }
    
    //check right
    for (let i = col+1; i < grid[row].length; i++){
        checkR++;
        if (cur <= grid[row][i]){
            break;
        }
    }
    //check top
    for (let i = row -1; i >= 0; i--){
        checkT++;
        if (cur <= grid[i][col]){
            break;
        }
    }
    //check bot
    for (let i = row+1; i < grid.length; i++){
        checkB++;
        if (cur <= grid[i][col]){
            break;
        }
    }

    return checkL * checkR * checkT * checkB;
}
let visCount = 0;
let highestScenicScore = 0;
for (let r = 0; r < grid.length; r++){
    for (let c = 0; c < grid[r].length; c++){
     
        let score = checkIsVisible(r, c, grid);
        if (score > highestScenicScore){
            highestScenicScore = score;
        }

    }
}


console.log(highestScenicScore)