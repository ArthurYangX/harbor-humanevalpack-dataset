# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*You are given two positive integers n and m, and your task is to compute the
  average of the integers from n through m (including n and m). 
  Round the answer to the nearest integer and convert that to binary.
  If n is greater than m, return -1.
  Example:
  roundedAvg(1, 5) => "0b11"
  roundedAvg(7, 5) => -1
  roundedAvg(10, 20) => "0b1111"
  roundedAvg(20, 33) => "0b11010"
  */
const roundedAvg = (n, m) => {
  if (n > m) { return -1 }
  let k = (n + m) / 2
  if (k % 1 != 0) { k = (n + m + 1) / 2 }
  return '0b' + k.toString(2)
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

/*You are given two positive integers n and m, and your task is to compute the
  average of the integers from n through m (including n and m). 
  Round the answer to the nearest integer and convert that to binary.
  If n is greater than m, return -1.
  Example:
  roundedAvg(1, 5) => "0b11"
  roundedAvg(7, 5) => -1
  roundedAvg(10, 20) => "0b1111"
  roundedAvg(20, 33) => "0b11010"
  */
const roundedAvg = (n, m) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
