# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* From a list of integers, remove all elements that occur more than once.
  Keep order of elements left the same as in the input.
  >>> removeDuplicates([1, 2, 3, 2, 4])
  [1, 3, 4]
  */
const removeDuplicates = (numbers) => {
  var dict = new Object();
  for (const num of numbers) {
    if (num in dict) {
      dict[num] += 1;
    } else {
      dict[num] = 1;
    }
  }
  return numbers.filter(x => dict[x] <= 1);
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

/* From a list of integers, remove all elements that occur more than once.
  Keep order of elements left the same as in the input.
  >>> removeDuplicates([1, 2, 3, 2, 4])
  [1, 3, 4]
  */
const removeDuplicates = (numbers) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
