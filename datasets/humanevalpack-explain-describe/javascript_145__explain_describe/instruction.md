# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  Write a function which sorts the given list of integers
  in ascending order according to the sum of their digits.
  Note: if there are several items with similar sum of their digits,
  order them based on their index in original list.

  For example:
  >>> orderByPoints([1, 11, -1, -11, -12]) == [-1, -11, 1, -12, 11]
  >>> orderByPoints([]) == []
  */
const orderByPoints = (nums) => {
  let p = nums
  for (let j = p.length - 2; j >= 0; j--) {
    for (let k = 0; k <= j; k++) {
      let m = 0
      let n = 0
      let h = p[k]
      let d = p[k + 1]
      let y = 1
      let u = 1
      if (h < 0) { y = -1; h = -h; }
      if (d < 0) { u = -1; d = -d; }
      while (h >= 10) {
        m += h % 10;
        h = (h - h % 10) / 10;
      }
      m += y * h
      while (d >= 10) {
        n += d % 10;
        d = (d - d % 10) / 10;
      }
      n += u * d
      if (m > n) {
        let tmp = p[k]
        p[k] = p[k + 1]
        p[k + 1] = tmp
      }
    }
  }
  return p
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
  Write a function which sorts the given list of integers
  in ascending order according to the sum of their digits.
  Note: if there are several items with similar sum of their digits,
  order them based on their index in original list.

  For example:
  >>> orderByPoints([1, 11, -1, -11, -12]) == [-1, -11, 1, -12, 11]
  >>> orderByPoints([]) == []
  */
const orderByPoints = (nums) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
