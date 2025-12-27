# Context

```javascript
/* Find how many times a given substring can be found in the original string. Count overlaping cases.
  >>> howManyTimes('', 'a')
  0
  >>> howManyTimes('aaa', 'a')
  3
  >>> howManyTimes('aaaa', 'aa')
  3
  */
const howManyTimes = (string, substring) => {
  var times = 0;
  for (let i = 0; i < string.length - substring.length; i++) {
    if (string.slice(i, i+substring.length) == substring) {
      times += 1;
    }
  }
  return times;
}
```

# Instruction

Fix bugs in howManyTimes.

# Prompt

/* Find how many times a given substring can be found in the original string. Count overlaping cases.
  >>> howManyTimes('', 'a')
  0
  >>> howManyTimes('aaa', 'a')
  3
  >>> howManyTimes('aaaa', 'aa')
  3
  */
const howManyTimes = (string, substring) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
