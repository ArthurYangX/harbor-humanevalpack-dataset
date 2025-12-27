# Context

```javascript
/*
  Given a positive integer n, return the count of the numbers of n-digit
  positive integers that start or end with 1.
  */
const startsOneEnds = (n) => {
  if (n == 1) { return 1 }
  let t = 18
  for (let i = 2; i < n; i++) {
    t = t * i * 10
  }
  return t
}
```

# Instruction

Fix bugs in startsOneEnds.

# Prompt

/*
  Given a positive integer n, return the count of the numbers of n-digit
  positive integers that start or end with 1.
  */
const startsOneEnds = (n) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
