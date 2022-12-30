const fs = require("fs");
const lines = fs.readFileSync("input.txt", "utf-8").split("\n");
const decrypt = (snafuString) => {
  let result = 0;
  for (let i = snafuString.length - 1; i >= 0; i--) {
    const power = snafuString.length - 1 - i;
    let digitValue = snafuString[i];
    let val = null;
    if (digitValue === "-") {
      val = -1;
    } else if (digitValue === "=") {
      val = -2;
    } else {
      val = parseInt(digitValue);
    }
    val *= Math.pow(5, power);
    result += val;
  }
  return result;
};

const encrypt = (value) => {
  let baseFiveList = value.toString(5).split("").map(Number);
  while (baseFiveList.includes(4) || baseFiveList.includes(3)) {
    for (let i = 0; i < baseFiveList.length; i++) {
      if (baseFiveList[i] === 4) {
        baseFiveList[i] = -1;
        baseFiveList[i - 1] += 1;
      } else if (baseFiveList[i] === 3) {
        baseFiveList[i] = -2;
        baseFiveList[i - 1] += 1;
      }
    }
  }

  return baseFiveList
    .map((num) => {
      if (num === -1) {
        return "-";
      } else if (num === -2) {
        return "=";
      } else {
        return num.toString();
      }
    })
    .join("");
};

const part1 = () => {
  let total = 0;
  lines.forEach((line) => {
    total += decrypt(line);
  });
  const result = encrypt(total);
  console.log("result:", result);
};

part1();
