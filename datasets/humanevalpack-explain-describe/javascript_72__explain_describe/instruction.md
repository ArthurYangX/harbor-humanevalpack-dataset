# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  Write a function that returns true if the object q will fly, and false otherwise.
  The object q will fly if it's balanced (it is a palindromic list) and the sum of its elements is less than or equal the maximum possible weight w.

  Example:
  willItFly([1, 2], 5) ➞ false
  # 1+2 is less than the maximum possible weight, but it's unbalanced.

  willItFly([3, 2, 3], 1) ➞ false
  # it's balanced, but 3+2+3 is more than the maximum possible weight.

  willItFly([3, 2, 3], 9) ➞ true
  # 3+2+3 is less than the maximum possible weight, and it's balanced.

  willItFly([3], 5) ➞ true
  # 3 is less than the maximum possible weight, and it's balanced.
  */
const willItFly = (q, w) => {
  if (q.reduce(((prev, item) => prev + item), 0) > w)
    return false;
  var i = 0, j = q.length - 1;
  while (i < j) {
    if (q[i] != q[j])
      return false;
    i++;
    j--;
  }
  return true;
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

/*
  Write a function that returns true if the object q will fly, and false otherwise.
  The object q will fly if it's balanced (it is a palindromic list) and the sum of its elements is less than or equal the maximum possible weight w.

  Example:
  willItFly([1, 2], 5) ➞ false
  # 1+2 is less than the maximum possible weight, but it's unbalanced.

  willItFly([3, 2, 3], 1) ➞ false
  # it's balanced, but 3+2+3 is more than the maximum possible weight.

  willItFly([3, 2, 3], 9) ➞ true
  # 3+2+3 is less than the maximum possible weight, and it's balanced.

  willItFly([3], 5) ➞ true
  # 3 is less than the maximum possible weight, and it's balanced.
  */
const willItFly = (q, w) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
