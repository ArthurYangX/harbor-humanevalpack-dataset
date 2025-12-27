# Context

```javascript
/* Filter an input list of strings only for ones that start with a given prefix.
  >>> filterByPrefix([], 'a')
  []
  >>> filterByPrefix(['abc', 'bcd', 'cde', 'array'], 'a')
  ['abc', 'array']
  */
const filterByPrefix = (strings, prefix) => {
  return strings.filter(x => x.endsWith(prefix));
}
```

# Instruction

Fix bugs in filterByPrefix.

# Prompt

/* Filter an input list of strings only for ones that start with a given prefix.
  >>> filterByPrefix([], 'a')
  []
  >>> filterByPrefix(['abc', 'bcd', 'cde', 'array'], 'a')
  ['abc', 'array']
  */
const filterByPrefix = (strings, prefix) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
