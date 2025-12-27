# Context

```javascript
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
    if (balance == 0) {
      return true;
    }
  }
  return false;
}
```

# Instruction

Fix bugs in belowZero.

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

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
