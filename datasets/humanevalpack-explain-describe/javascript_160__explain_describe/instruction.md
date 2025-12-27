# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  Given two lists operator, and operand. The first list has basic algebra operations, and 
  the second list is a list of integers. Use the two given lists to build the algebric 
  expression and return the evaluation of this expression.

  The basic algebra operations:
  Addition ( + ) 
  Subtraction ( - ) 
  Multiplication ( * ) 
  Floor division ( // ) 
  Exponentiation ( ** ) 

  Example:
  operator['+', '*', '-']
  array = [2, 3, 4, 5]
  result = 2 + 3 * 4 - 5
  => result = 9

  Note:
      The length of operator list is equal to the length of operand list minus one.
      Operand is a list of of non-negative integers.
      Operator list has at least one operator, and operand list has at least two operands.

  */
const doAlgebra = (operator, operand) => {
  while (operator.length > 0) {
    let y = 0
    for (let i = operator.length - 1; i >= 0; i--) {
      if (operator[i] == '**') {
        let u = operand[i]
        while (operand[i + 1] > 1) {
          operand[i + 1]--;
          operand[i] *= u;
        }
        operand.splice(i + 1, 1)
        operator.splice(i, 1)
        y = 1;
        break;
      }
    }
    if (y == 1) { continue }
    for (let i = 0; i < operator.length; i++) {
      if (operator[i] == '*') {
        operand[i] *= operand[i + 1]
        operand.splice(i + 1, 1)
        operator.splice(i, 1)
        y = 1;
        break;
      }
      else if (operator[i] == '//') {
        operand[i] = (operand[i] - operand[i] % operand[i + 1]) / operand[i + 1]
        operand.splice(i + 1, 1)
        operator.splice(i, 1)
        y = 1;
        break;
      }
    }
    if (y == 1) { continue }
    for (let i = 0; i < operator.length; i++) {
      if (operator[i] == '+') {
        operand[i] += operand[i + 1]
        operand.splice(i + 1, 1)
        operator.splice(i, 1)
        y = 1;
        break;
      }
      else if (operator[i] == '-') {
        operand[i] -= operand[i + 1]
        operand.splice(i + 1, 1)
        operator.splice(i, 1)
        y = 1;
        break;
      }
    }
    if (y == 1) { continue }
  }
  return operand[0]
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
  Given two lists operator, and operand. The first list has basic algebra operations, and 
  the second list is a list of integers. Use the two given lists to build the algebric 
  expression and return the evaluation of this expression.

  The basic algebra operations:
  Addition ( + ) 
  Subtraction ( - ) 
  Multiplication ( * ) 
  Floor division ( // ) 
  Exponentiation ( ** ) 

  Example:
  operator['+', '*', '-']
  array = [2, 3, 4, 5]
  result = 2 + 3 * 4 - 5
  => result = 9

  Note:
      The length of operator list is equal to the length of operand list minus one.
      Operand is a list of of non-negative integers.
      Operator list has at least one operator, and operand list has at least two operands.

  */
const doAlgebra = (operator, operand) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
