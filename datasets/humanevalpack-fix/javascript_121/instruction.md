# Context

```javascript
/*Given a non-empty list of integers, return the sum of all of the odd elements that are in even positions.
  

  Examples
  solution([5, 8, 7, 1]) ==> 12
  solution([3, 3, 3, 3, 3]) ==> 9
  solution([30, 13, 24, 321]) ==>0
  */
const solution = (lst) => {
  let p = 1
  for (let i = 0; i < lst.length; i += 2) {
    if (lst[i] % 2 == 1) {
      p += lst[i]
    }
  }
  return p
}
```

# Instruction

Fix bugs in solution.

# Prompt

/*Given a non-empty list of integers, return the sum of all of the odd elements that are in even positions.
  

  Examples
  solution([5, 8, 7, 1]) ==> 12
  solution([3, 3, 3, 3, 3]) ==> 9
  solution([30, 13, 24, 321]) ==>0
  */
const solution = (lst) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
