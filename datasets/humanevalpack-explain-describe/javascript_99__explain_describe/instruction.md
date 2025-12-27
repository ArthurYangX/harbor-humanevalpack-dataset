# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Create a function that takes a value (string) representing a number
  and returns the closest integer to it. If the number is equidistant
  from two integers, round it away from zero.
  Examples
  >>> closestInteger("10")
  10
  >>> closestInteger("15.3")
  15
  Note:
  Rounding away from zero means that if the given number is equidistant
  from two integers, the one you should return is the one that is the
  farthest from zero. For example closestInteger("14.5") should
  return 15 and closestInteger("-14.5") should return -15.
  */
const closestInteger = (value) => {
  value = Number(value)
  let t = value % 1
  if (t < 0.5 && t > -0.5) { value -= t }
  else { value += t }
  return value
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

/* Create a function that takes a value (string) representing a number
  and returns the closest integer to it. If the number is equidistant
  from two integers, round it away from zero.
  Examples
  >>> closestInteger("10")
  10
  >>> closestInteger("15.3")
  15
  Note:
  Rounding away from zero means that if the given number is equidistant
  from two integers, the one you should return is the one that is the
  farthest from zero. For example closestInteger("14.5") should
  return 15 and closestInteger("-14.5") should return -15.
  */
const closestInteger = (value) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
