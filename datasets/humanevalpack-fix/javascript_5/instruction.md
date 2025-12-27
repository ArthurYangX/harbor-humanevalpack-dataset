# Context

```javascript
/* Insert a number 'delimeter' between every two consecutive elements of input list `numbers'
  >>> intersperse([], 4)
  []
  >>> intersperse([1, 2, 3], 4)
  [1, 4, 2, 4, 3]
  */
const intersperse = (numbers, delimeter) => {
  if (!Array.isArray(numbers) || numbers.length == 0)
    return [];
  var result = [];
  for (const n of numbers) {
    result.push(n, delimeter);
  }
  return result;
}
```

# Instruction

Fix bugs in intersperse.

# Prompt

/* Insert a number 'delimeter' between every two consecutive elements of input list `numbers'
  >>> intersperse([], 4)
  []
  >>> intersperse([1, 2, 3], 4)
  [1, 4, 2, 4, 3]
  */
const intersperse = (numbers, delimeter) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
