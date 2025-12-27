# Context

```javascript
/* Given a string, find out how many distinct characters (regardless of case) does it consist of
  >>> countDistinctCharacters('xyzXYZ')
  3
  >>> countDistinctCharacters('Jerry')
  4
  */
const countDistinctCharacters = (string) => {
  return (new Set(string)).size;

}
```

# Instruction

Fix bugs in countDistinctCharacters.

# Prompt

/* Given a string, find out how many distinct characters (regardless of case) does it consist of
  >>> countDistinctCharacters('xyzXYZ')
  3
  >>> countDistinctCharacters('Jerry')
  4
  */
const countDistinctCharacters = (string) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
