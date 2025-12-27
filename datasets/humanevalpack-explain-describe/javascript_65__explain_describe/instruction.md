# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Circular shift the digits of the integer x, shift the digits right by shift
  and return the result as a string.
  If shift > number of digits, return digits reversed.
  >>> circularShift(12, 1)
  "21"
  >>> circularShift(12, 2)
  "12"
  */
const circularShift = (x, shift) => {
  s = x.toString();
  if (shift > s.length)
    return s.split('').reverse().join('');
  else
    return s.slice(-shift) + s.slice(0, -shift);
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

/*Circular shift the digits of the integer x, shift the digits right by shift
  and return the result as a string.
  If shift > number of digits, return digits reversed.
  >>> circularShift(12, 1)
  "21"
  >>> circularShift(12, 2)
  "12"
  */
const circularShift = (x, shift) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
