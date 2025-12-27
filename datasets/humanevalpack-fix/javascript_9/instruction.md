# Context

```javascript
/* From a given list of integers, generate a list of rolling maximum element found until given moment
  in the sequence.
  >>> rollingMax([1, 2, 3, 2, 3, 4, 2])
  [1, 2, 3, 3, 3, 4, 4]
  */
const rollingMax = (numbers) => {
  var running_max, result = [];
  for (const n of numbers) {
    if (running_max == undefined)
      running_max = n;
    else
      running_max = Math.max(numbers);
    result.push(running_max);
  }
  return result;
}
```

# Instruction

Fix bugs in rollingMax.

# Prompt

/* From a given list of integers, generate a list of rolling maximum element found until given moment
  in the sequence.
  >>> rollingMax([1, 2, 3, 2, 3, 4, 2])
  [1, 2, 3, 3, 3, 4, 4]
  */
const rollingMax = (numbers) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
