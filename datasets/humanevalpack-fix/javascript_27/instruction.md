# Context

```javascript
/* For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
  >>> flipCase('Hello')
  'hELLO'
  */
const flipCase = (string) => {
  return string.split('')
          .map(x => (x.toLowerCase() == x ? x.toLowerCase() : x.toUpperCase()))
          .join('');
}
```

# Instruction

Fix bugs in flipCase.

# Prompt

/* For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
  >>> flipCase('Hello')
  'hELLO'
  */
const flipCase = (string) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
