# Context

You are given a reference implementation (canonical solution) to explain.

```js
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
  for (let i = 0; i < string.length - substring.length + 1; i++) {
    if (string.slice(i, i+substring.length) == substring) {
      times += 1;
    }
  }
  return times;
}
```

# Instruction

Explain the algorithm in natural language:
- Describe the core idea and step-by-step approach
- Specify inputs/outputs and edge cases
- Provide time/space complexity

Constraints:
- Output must be plain text only
- Do NOT output any code
- Write your explanation to `solution/explanation.txt`

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

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
