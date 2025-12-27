# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*This function takes a list l and returns a list l' such that
  l' is identical to l in the odd indicies, while its values at the even indicies are equal
  to the values of the even indicies of l, but sorted.
  >>> sortEven([1, 2, 3])
  [1, 2, 3]
  >>> sortEven([5, 6, 3, 4])
  [3, 6, 5, 4]
  */
const sortEven = (l) => {
  var even = l.filter((item, index) => index % 2 == 0);
  even.sort((a, b) => (a - b));
  return l.map((item, index) => (index % 2 == 0 ? even[index / 2] : item));
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

/*This function takes a list l and returns a list l' such that
  l' is identical to l in the odd indicies, while its values at the even indicies are equal
  to the values of the even indicies of l, but sorted.
  >>> sortEven([1, 2, 3])
  [1, 2, 3]
  >>> sortEven([5, 6, 3, 4])
  [3, 6, 5, 4]
  */
const sortEven = (l) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
