# Context

```javascript
/* Filter given list of any python values only for integers
  >>> filterIntegers(['a', 3.14, 5])
  [5]
  >>> filterIntegers([1, 2, 3, 'abc', {}, []])
  [1, 2, 3]
  */
const filterIntegers = (values) => {
  values.filter(x => Number.isInteger(x));
  return values;
}
```

# Instruction

Fix bugs in filterIntegers.

# Prompt

/* Filter given list of any python values only for integers
  >>> filterIntegers(['a', 3.14, 5])
  [5]
  >>> filterIntegers([1, 2, 3, 'abc', {}, []])
  [1, 2, 3]
  */
const filterIntegers = (values) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
