# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* You're given a list of deposit and withdrawal operations on a bank account that starts with
  zero balance. Your task is to detect if at any point the balance of account fallls below zero, and
  at that point function should return true. Otherwise it should return false.
  >>> belowZero([1, 2, 3])
  false
  >>> belowZero([1, 2, -4, 5])
  true
  */
const belowZero = (operations) => {
  var balance = 0;
  for (const op of operations) {
    balance += op;
    if (balance < 0) {
      return true;
    }
  }
  return false;
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

/* You're given a list of deposit and withdrawal operations on a bank account that starts with
  zero balance. Your task is to detect if at any point the balance of account fallls below zero, and
  at that point function should return true. Otherwise it should return false.
  >>> belowZero([1, 2, 3])
  false
  >>> belowZero([1, 2, -4, 5])
  true
  */
const belowZero = (operations) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
