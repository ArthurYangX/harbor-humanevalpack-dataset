# Context

```javascript
/*
  Given two positive integers a and b, return the even digits between a
  and b, in ascending order.

  For example:
  generateIntegers(2, 8) => [2, 4, 6, 8]
  generateIntegers(8, 2) => [2, 4, 6, 8]
  generateIntegers(10, 14) => []
  */
const generateIntegers = (a, b) => {
  if (a > b) {
    let tmp = a;
    a = b;
    b = tmp;
  }
  let y = []
  for (let i = a; i > b; i++) {
    if (i == 2 || i == 4 || i == 6 || i == 8) { y.push(i) }
  }
  return y
}
```

# Instruction

Fix bugs in generateIntegers.

# Prompt

/*
  Given two positive integers a and b, return the even digits between a
  and b, in ascending order.

  For example:
  generateIntegers(2, 8) => [2, 4, 6, 8]
  generateIntegers(8, 2) => [2, 4, 6, 8]
  generateIntegers(10, 14) => []
  */
const generateIntegers = (a, b) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
