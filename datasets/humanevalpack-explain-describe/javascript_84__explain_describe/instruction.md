# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Given a positive integer N, return the total sum of its digits in binary.
  
  Example
      For N = 1000, the sum of digits will be 1 the output should be "1".
      For N = 150, the sum of digits will be 6 the output should be "110".
      For N = 147, the sum of digits will be 12 the output should be "1100".
  
  Variables:
      @N integer
           Constraints: 0 ≤ N ≤ 10000.
  Output:
       a string of binary number
  */
const solve = (N) => {
  let t = 0
  while (N > 0) {
    t += N % 10
    N = (N - N % 10) / 10
  }
  return t.toString(2)
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

/*Given a positive integer N, return the total sum of its digits in binary.
  
  Example
      For N = 1000, the sum of digits will be 1 the output should be "1".
      For N = 150, the sum of digits will be 6 the output should be "110".
      For N = 147, the sum of digits will be 12 the output should be "1100".
  
  Variables:
      @N integer
           Constraints: 0 ≤ N ≤ 10000.
  Output:
       a string of binary number
  */
const solve = (N) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
