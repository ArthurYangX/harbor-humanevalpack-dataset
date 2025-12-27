# Context

```javascript
/* Return a greatest common divisor of two integers a and b
  >>> greatestCommonDivisor(3, 5)
  1
  >>> greatestCommonDivisor(25, 15)
  5
  */
const greatestCommonDivisor = (a, b) => {
  while (b != 0) {
    let temp = a;
    a = b;
    b = temp % b;
  }
  return b;
}
```

# Instruction

Fix bugs in greatestCommonDivisor.

# Prompt

/* Return a greatest common divisor of two integers a and b
  >>> greatestCommonDivisor(3, 5)
  1
  >>> greatestCommonDivisor(25, 15)
  5
  */
const greatestCommonDivisor = (a, b) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
