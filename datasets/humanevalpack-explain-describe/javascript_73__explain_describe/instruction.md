# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  Given an array arr of integers, find the minimum number of elements that
  need to be changed to make the array palindromic. A palindromic array is an array that
  is read the same backwards and forwards. In one change, you can change one element to any other element.

  For example:
  smallestChange([1,2,3,5,4,7,9,6]) == 4
  smallestChange([1, 2, 3, 4, 3, 2, 2]) == 1
  smallestChange([1, 2, 3, 2, 1]) == 0
  */
const smallestChange = (arr) => {
  var ans = 0;
  for (let i = 0; i < Math.floor(arr.length / 2); i++)
    if (arr[i] != arr.at(-i - 1))
      ans++;
  return ans;
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
  Given an array arr of integers, find the minimum number of elements that
  need to be changed to make the array palindromic. A palindromic array is an array that
  is read the same backwards and forwards. In one change, you can change one element to any other element.

  For example:
  smallestChange([1,2,3,5,4,7,9,6]) == 4
  smallestChange([1, 2, 3, 4, 3, 2, 2]) == 1
  smallestChange([1, 2, 3, 2, 1]) == 0
  */
const smallestChange = (arr) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
