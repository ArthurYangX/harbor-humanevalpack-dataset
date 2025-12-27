# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Your task is to write a function that returns true if a number x is a simple
  power of n and false in other cases.
  x is a simple power of n if n**int=x
  For example:
  isSimplePower(1, 4) => true
  isSimplePower(2, 2) => true
  isSimplePower(8, 2) => true
  isSimplePower(3, 2) => false
  isSimplePower(3, 1) => false
  isSimplePower(5, 3) => false
  */
const isSimplePower = (x, n) => {
  if (n == 1)
    return (x == 1);
  var power = 1;
  while (power < x)
    power = power * n;
  return (power == x);
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

/*Your task is to write a function that returns true if a number x is a simple
  power of n and false in other cases.
  x is a simple power of n if n**int=x
  For example:
  isSimplePower(1, 4) => true
  isSimplePower(2, 2) => true
  isSimplePower(8, 2) => true
  isSimplePower(3, 2) => false
  isSimplePower(3, 1) => false
  isSimplePower(5, 3) => false
  */
const isSimplePower = (x, n) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
