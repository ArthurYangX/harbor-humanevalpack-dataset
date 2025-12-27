# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  Given list of integers, return list in strange order.
  Strange sorting, is when you start with the minimum value,
  then maximum of the remaining integers, then minimum and so on.

  Examples:
  strangeSortList([1, 2, 3, 4]) == [1, 4, 2, 3]
  strangeSortList([5, 5, 5, 5]) == [5, 5, 5, 5]
  strangeSortList([]) == []
  */
const strangeSortList = (lst) => {
  var res = [], sw = true;
  while (lst.length) {
    res.push(sw ? Math.min(...lst) : Math.max(...lst));
    lst.splice(lst.indexOf(res.at(-1)), 1);
    sw = !sw;
  }
  return res;
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
  Given list of integers, return list in strange order.
  Strange sorting, is when you start with the minimum value,
  then maximum of the remaining integers, then minimum and so on.

  Examples:
  strangeSortList([1, 2, 3, 4]) == [1, 4, 2, 3]
  strangeSortList([5, 5, 5, 5]) == [5, 5, 5, 5]
  strangeSortList([]) == []
  */
const strangeSortList = (lst) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
