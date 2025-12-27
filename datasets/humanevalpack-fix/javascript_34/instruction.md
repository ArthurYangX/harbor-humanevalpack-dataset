# Context

```javascript
/*Return sorted unique elements in a list
  >>> unique([5, 3, 5, 2, 3, 3, 9, 0, 123])
  [0, 2, 3, 5, 9, 123]
  */
const unique = (l) => {
  return l.sort((a, b) => (a - b));
}
```

# Instruction

Fix bugs in unique.

# Prompt

/*Return sorted unique elements in a list
  >>> unique([5, 3, 5, 2, 3, 3, 9, 0, 123])
  [0, 2, 3, 5, 9, 123]
  */
const unique = (l) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
