# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*This function takes two positive numbers x and y and returns the
  biggest even integer number that is in the range [x, y] inclusive. If 
  there's no such number, then the function should return -1.

  For example:
  chooseNum(12, 15) = 14
  chooseNum(13, 12) = -1
  */
const chooseNum = (x, y) => {
  for (let i = y; i >= x; i--) {
    if (i % 2 == 0) {return i }
  }
  return -1
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

/*This function takes two positive numbers x and y and returns the
  biggest even integer number that is in the range [x, y] inclusive. If 
  there's no such number, then the function should return -1.

  For example:
  chooseNum(12, 15) = 14
  chooseNum(13, 12) = -1
  */
const chooseNum = (x, y) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
