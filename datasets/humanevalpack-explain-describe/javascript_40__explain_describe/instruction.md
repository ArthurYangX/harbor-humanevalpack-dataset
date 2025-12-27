# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  triplesSumToZero takes a list of integers as an input.
  it returns true if there are three distinct elements in the list that
  sum to zero, and false otherwise.

  >>> triplesSumToZero([1, 3, 5, 0])
  false
  >>> triplesSumToZero([1, 3, -2, 1])
  true
  >>> triplesSumToZero([1, 2, 3, 7])
  false
  >>> triplesSumToZero([2, 4, -5, 3, 9, 7])
  true
  >>> triplesSumToZero([1])
  false
  */
const triplesSumToZero = (l) => {
  for (let i = 0; i < l.length; i++)
    for (let j = i + 1; j < l.length; j++)
      for (let k = j + 1; k < l.length; k++)
        if (l[i] + l[j] + l[k] == 0)
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
  triplesSumToZero takes a list of integers as an input.
  it returns true if there are three distinct elements in the list that
  sum to zero, and false otherwise.

  >>> triplesSumToZero([1, 3, 5, 0])
  false
  >>> triplesSumToZero([1, 3, -2, 1])
  true
  >>> triplesSumToZero([1, 2, 3, 7])
  false
  >>> triplesSumToZero([2, 4, -5, 3, 9, 7])
  true
  >>> triplesSumToZero([1])
  false
  */
const triplesSumToZero = (l) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
