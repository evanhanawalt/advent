const fs = require("fs");
const lines = fs.readFileSync("input.txt", "utf-8").split("\n");

const part1 = () => {
    const m = new Map();
    lines.forEach(line => {
        const split = line.split(":");
        const key = split[0];
        if (!isNaN(parseInt(split[1]))) {
            m.set(key, parseInt(split[1]));
        } else {
            const equation = split[1].trim().split(" ");
            const vals = {
                left: equation[0],
                operation: equation[1],
                right: equation[2]
            }
            m.set(key, vals);
        }
    });

    const getResult = (token) => {
        const value = m.get(token);
        if (typeof value === 'number') {
            return value;
        } else if (value.operation === "/") {
            const calc = getResult(value.left) / getResult(value.right);
            m.set(token, calc);
            return calc;
        } else if (value.operation === "+") {
            const calc = getResult(value.left) + getResult(value.right);
            m.set(token, calc);
            return calc;
        } else if (value.operation === "*") {
            const calc = getResult(value.left) * getResult(value.right);
            m.set(token, calc);
            return calc;
        } else if (value.operation === "-") {
            const calc = getResult(value.left) - getResult(value.right);
            m.set(token, calc);
            return calc;
        } else {
            throw new Error("this shouldn't happen");
        }
    }

    console.log("Part1: ", getResult("root"));
}

part1();



const part2 = () => {
    const m = new Map();
    lines.forEach(line => {
        const split = line.split(":");
        const key = split[0];
        if (key !== "humn") {
            if (!isNaN(parseInt(split[1]))) {
                m.set(key, parseInt(split[1]));
            } else {
                let [left, operation, right] = split[1].trim().split(" ");
                if (key === "root") {
                    operation = "==="
                }
                const vals = {
                    left,
                    operation,
                    right
                };
                m.set(key, vals);
            }
        }
    });


    m.set("humn", "humn");
    const getResult = (token) => {
        const value = m.get(token);
        if (typeof value === 'number') {
            return value;
        } else if (value === 'humn') {
            return value;
        } else {
            const leftValue = getResult(value.left);
            const rightValue = getResult(value.right);


            if (typeof leftValue !== "number" || typeof rightValue !== "number") {
                return {
                    left: leftValue,
                    operation: value.operation,
                    right: rightValue
                };
            } else {
                if (value.operation === "/") {
                    const calc = getResult(value.left) / getResult(value.right);
                    m.set(token, calc);
                    return calc;
                } else if (value.operation === "+") {
                    const calc = getResult(value.left) + getResult(value.right);
                    m.set(token, calc);
                    return calc;
                } else if (value.operation === "*") {
                    const calc = getResult(value.left) * getResult(value.right);
                    m.set(token, calc);
                    return calc;
                } else if (value.operation === "-") {
                    const calc = getResult(value.left) - getResult(value.right);
                    m.set(token, calc);
                    return calc;
                } else {
                    throw new Error("this shouldn't happen");
                }
            }

        }
    }

    const result = getResult("root");
    let number = null;
    let equation = null;
    if (typeof result.left === "number") {
        number = result.left;
        equation = result.right;
    } else {
        number = result.right;
        equation = result.left;
    }
    // unwind the equation to solve for humn
    while (typeof equation === "object") {
        const { left, operation, right } = equation;
        const leftIsNumber = typeof left === "number";
        const parameter = leftIsNumber ? left : right;
        const newEquation = leftIsNumber ? right : left;
        if (operation === "*") {
            number = number / parameter;
        } else if (operation === "+") {
            number = number - parameter;
        } else if (operation === "-") {
            if (leftIsNumber) {
                number = -1 * (number - parameter)
            } else {
                number = number + parameter;
            }

        } else if (operation === "/") {
            if (leftIsNumber) {
                number = parameter / number;
            } else {
                number = number * parameter
            }
        }
        equation = newEquation;
    }
    console.log("Part2: ", number);
}

part2();