# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*The Brazilian factorial is defined as:
  brazilian_factorial(n) = n! * (n-1)! * (n-2)! * ... * 1!
  where n > 0

  For example:
  >>> specialFactorial(4)
  288

  The function will receive an integer as input and should return the special
  factorial of this integer.
  */
const specialFactorial = (n) => {
  let p = 1;
  let t = 1;
  while (n > 1) {
    let y = p;
    while (y > 0) {
      y--;
      t *= n;
    }
    p++;
    n--;
  }
  return t
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

/*The Brazilian factorial is defined as:
  brazilian_factorial(n) = n! * (n-1)! * (n-2)! * ... * 1!
  where n > 0

  For example:
  >>> specialFactorial(4)
  288

  The function will receive an integer as input and should return the special
  factorial of this integer.
  */
const specialFactorial = (n) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
