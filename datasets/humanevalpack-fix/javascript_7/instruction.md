# Context

```javascript
/* Filter an input list of strings only for ones that contain given substring
  >>> filterBySubstring([], 'a')
  []
  >>> filterBySubstring(['abc', 'bacd', 'cde', 'array'], 'a')
  ['abc', 'bacd', 'array']
  */
const filterBySubstring = (strings, substring) => {
  return strings.filter(x => substring.indexOf(x) != -1);
}
```

# Instruction

Fix bugs in filterBySubstring.

# Prompt

/* Filter an input list of strings only for ones that contain given substring
  >>> filterBySubstring([], 'a')
  []
  >>> filterBySubstring(['abc', 'bacd', 'cde', 'array'], 'a')
  ['abc', 'bacd', 'array']
  */
const filterBySubstring = (strings, substring) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
