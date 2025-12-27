# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  pairsSumToZero takes a list of integers as an input.
  it returns true if there are two distinct elements in the list that
  sum to zero, and false otherwise.
  >>> pairsSumToZero([1, 3, 5, 0])
  false
  >>> pairsSumToZero([1, 3, -2, 1])
  false
  >>> pairsSumToZero([1, 2, 3, 7])
  false
  >>> pairsSumToZero([2, 4, -5, 3, 5, 7])
  true
  >>> pairsSumToZero([1])
  false
  */
const pairsSumToZero = (l) => {
  for (let i = 0; i < l.length; i++)
    for (let j = i + 1; j < l.length; j++)
      if (l[i] + l[j] == 0)
        return true;
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

/*
  pairsSumToZero takes a list of integers as an input.
  it returns true if there are two distinct elements in the list that
  sum to zero, and false otherwise.
  >>> pairsSumToZero([1, 3, 5, 0])
  false
  >>> pairsSumToZero([1, 3, -2, 1])
  false
  >>> pairsSumToZero([1, 2, 3, 7])
  false
  >>> pairsSumToZero([2, 4, -5, 3, 5, 7])
  true
  >>> pairsSumToZero([1])
  false
  */
const pairsSumToZero = (l) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
