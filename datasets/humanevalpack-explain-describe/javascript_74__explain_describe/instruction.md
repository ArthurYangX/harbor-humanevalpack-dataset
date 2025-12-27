# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  Write a function that accepts two lists of strings and returns the list that has
  total number of chars in the all strings of the list less than the other list.

  if the two lists have the same number of chars, return the first list.

  Examples
  totalMatch([], []) ➞ []
  totalMatch(['hi', 'admin'], ['hI', 'Hi']) ➞ ['hI', 'Hi']
  totalMatch(['hi', 'admin'], ['hi', 'hi', 'admin', 'project']) ➞ ['hi', 'admin']
  totalMatch(['hi', 'admin'], ['hI', 'hi', 'hi']) ➞ ['hI', 'hi', 'hi']
  totalMatch(['4'], ['1', '2', '3', '4', '5']) ➞ ['4']
  */
const totalMatch = (lst1, lst2) => {
  var l1 = lst1.reduce(((prev, item) => prev + item.length), 0);
  var l2 = lst2.reduce(((prev, item) => prev + item.length), 0);
  if (l1 <= l2)
    return lst1;
  else
    return lst2;
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
  Write a function that accepts two lists of strings and returns the list that has
  total number of chars in the all strings of the list less than the other list.

  if the two lists have the same number of chars, return the first list.

  Examples
  totalMatch([], []) ➞ []
  totalMatch(['hi', 'admin'], ['hI', 'Hi']) ➞ ['hI', 'Hi']
  totalMatch(['hi', 'admin'], ['hi', 'hi', 'admin', 'project']) ➞ ['hi', 'admin']
  totalMatch(['hi', 'admin'], ['hI', 'hi', 'hi']) ➞ ['hI', 'hi', 'hi']
  totalMatch(['4'], ['1', '2', '3', '4', '5']) ➞ ['4']
  */
const totalMatch = (lst1, lst2) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
