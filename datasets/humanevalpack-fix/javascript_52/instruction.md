# Context

```javascript
/*Return true if all numbers in the list l are below threshold t.
  >>> belowThreshold([1, 2, 4, 10], 100)
  true
  >>> belowThreshold([1, 20, 4, 10], 5)
  false
  */
const belowThreshold = (l, t) => {
  for (const e of l)
    if (e >= t)
      return true;
  return false;
}
```

# Instruction

Fix bugs in belowThreshold.

# Prompt

/*Return true if all numbers in the list l are below threshold t.
  >>> belowThreshold([1, 2, 4, 10], 100)
  true
  >>> belowThreshold([1, 20, 4, 10], 5)
  false
  */
const belowThreshold = (l, t) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
