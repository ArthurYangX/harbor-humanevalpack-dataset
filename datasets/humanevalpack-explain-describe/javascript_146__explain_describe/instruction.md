# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Write a function that takes an array of numbers as input and returns 
  the number of elements in the array that are greater than 10 and both 
  first and last digits of a number are odd (1, 3, 5, 7, 9).
  For example:
  specialFilter([15, -73, 14, -15]) => 1 
  specialFilter([33, -2, -3, 45, 21, 109]) => 2
  */
const specialFilter = (nums) => {
  let p = 0
  for (let i = 0; i < nums.length; i++) {
    if (nums[i] < 10) { continue }
    let y = nums[i].toString()
    if (Number(y[0]) % 2 == 1 && Number(y[y.length - 1]) % 2 == 1) {
      p++
    }
  }
  return p
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

/*Write a function that takes an array of numbers as input and returns 
  the number of elements in the array that are greater than 10 and both 
  first and last digits of a number are odd (1, 3, 5, 7, 9).
  For example:
  specialFilter([15, -73, 14, -15]) => 1 
  specialFilter([33, -2, -3, 45, 21, 109]) => 2
  */
const specialFilter = (nums) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
