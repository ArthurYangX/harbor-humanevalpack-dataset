# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Given a non-empty list of integers lst. add the even elements that are at odd indices..


  Examples:
      add([4, 2, 6, 7]) ==> 2 
  */
const add = (lst) => {
  let t = 0
  for (let i = 1; i < lst.length; i += 2) {
    if (lst[i] % 2 == 0) {
      t += lst[i]
    }
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

/*Given a non-empty list of integers lst. add the even elements that are at odd indices..


  Examples:
      add([4, 2, 6, 7]) ==> 2 
  */
const add = (lst) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
