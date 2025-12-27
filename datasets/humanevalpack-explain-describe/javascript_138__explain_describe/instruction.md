# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Evaluate whether the given number n can be written as the sum of exactly 4 positive even numbers
  Example
  isEqualToSumEven(4) == false
  isEqualToSumEven(6) == false
  isEqualToSumEven(8) == true
  */
const isEqualToSumEven = (n) => {
  return (n >= 8 && n % 2 == 0)
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

/*Evaluate whether the given number n can be written as the sum of exactly 4 positive even numbers
  Example
  isEqualToSumEven(4) == false
  isEqualToSumEven(6) == false
  isEqualToSumEven(8) == true
  */
const isEqualToSumEven = (n) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
