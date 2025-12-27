# Context

```javascript
/* For a given number n, find the largest number that divides n evenly, smaller than n
  >>> largestDivisor(15)
  5
  */
const largestDivisor = (n) => {
  for (let i = n - 1; i >= 0; i--)
    if (n - i == 0)
      return i;
}
```

# Instruction

Fix bugs in largestDivisor.

# Prompt

/* For a given number n, find the largest number that divides n evenly, smaller than n
  >>> largestDivisor(15)
  5
  */
const largestDivisor = (n) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
