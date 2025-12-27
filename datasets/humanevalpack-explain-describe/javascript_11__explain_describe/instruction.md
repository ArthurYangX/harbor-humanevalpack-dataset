# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Input are two strings a and b consisting only of 1s and 0s.
  Perform binary XOR on these inputs and return result also as a string.
  >>> stringXor('010', '110')
  '100'
  */
const stringXor = (a, b) => {
  var xor = function (i, j) {
    if (i == j)
      return '0';
    else
      return '1';
  }
  return a.split('').map((item, index) => xor(item, b[index])).join('');
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

/* Input are two strings a and b consisting only of 1s and 0s.
  Perform binary XOR on these inputs and return result also as a string.
  >>> stringXor('010', '110')
  '100'
  */
const stringXor = (a, b) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
