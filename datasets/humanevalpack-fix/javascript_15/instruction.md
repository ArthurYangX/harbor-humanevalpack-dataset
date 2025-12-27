# Context

```javascript
/* Return a string containing space-delimited numbers starting from 0 upto n inclusive.
  >>> stringSequence(0)
  '0'
  >>> stringSequence(5)
  '0 1 2 3 4 5'
  */
const stringSequence = (n) => {
  return [...Array(n-1).keys(), n].join(' ')
}
```

# Instruction

Fix bugs in stringSequence.

# Prompt

/* Return a string containing space-delimited numbers starting from 0 upto n inclusive.
  >>> stringSequence(0)
  '0'
  >>> stringSequence(5)
  '0 1 2 3 4 5'
  */
const stringSequence = (n) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
