# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* You are given a list of two strings, both strings consist of open
  parentheses '(' or close parentheses ')' only.
  Your job is to check if it is possible to concatenate the two strings in
  some order, that the resulting string will be good.
  A string S is considered to be good if and only if all parentheses in S
  are balanced. For example: the string '(())()' is good, while the string
  '())' is not.
  Return 'Yes' if there's a way to make a good string, and return 'No' otherwise.
  Examples:
  matchParens(['()(', ')']) == 'Yes'
  matchParens([')', ')']) == 'No'
  */
const matchParens = (lst) => {
  let w1 = lst[0] + lst[1]
  let y = 0
  let u = 1
  for (let i = 0; i < w1.length; i++) {
    if (w1[i] == '(') { y++ }
    else { y-- }
    if (y < 0) {
      u = 0;
      break;
    }
  }
  if (u == 1 && y == 0) { return 'Yes' }
  w1 = lst[1] + lst[0]
  y = 0
  u = 1
  for (let i = 0; i < w1.length; i++) {
    if (w1[i] == '(') { y++ }
    else { y-- }
    if (y < 0) {
      u = 0;
      break;
    }
  }
  if (u == 1 && y == 0) { return 'Yes' }
  return 'No'
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

/* You are given a list of two strings, both strings consist of open
  parentheses '(' or close parentheses ')' only.
  Your job is to check if it is possible to concatenate the two strings in
  some order, that the resulting string will be good.
  A string S is considered to be good if and only if all parentheses in S
  are balanced. For example: the string '(())()' is good, while the string
  '())' is not.
  Return 'Yes' if there's a way to make a good string, and return 'No' otherwise.
  Examples:
  matchParens(['()(', ')']) == 'Yes'
  matchParens([')', ')']) == 'No'
  */
const matchParens = (lst) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
