# Context

```javascript
/* Return list of all prefixes from shortest to longest of the input string
  >>> allPrefixes('abc')
  ['a', 'ab', 'abc']
  */
const allPrefixes = (string) => {
  var result = [];
  for (let i = 0; i < string.length-1; i++) {
    result.push(string.slice(0, i+1));
  }
  return result;
}
```

# Instruction

Fix bugs in allPrefixes.

# Prompt

/* Return list of all prefixes from shortest to longest of the input string
  >>> allPrefixes('abc')
  ['a', 'ab', 'abc']
  */
const allPrefixes = (string) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
