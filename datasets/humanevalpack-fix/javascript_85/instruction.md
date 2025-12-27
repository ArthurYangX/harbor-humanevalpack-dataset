# Context

```javascript
/*Given a non-empty list of integers lst. add the even elements that are at odd indices..


  Examples:
      add([4, 2, 6, 7]) ==> 2 
  */
const add = (lst) => {
  let t = 0
  for (let i = 1; i < lst.length; i += 1) {
    if (lst[i] % 2 == 0) {
      t += lst[i]
    }
  }
  return t
}
```

# Instruction

Fix bugs in add.

# Prompt

/*Given a non-empty list of integers lst. add the even elements that are at odd indices..


  Examples:
      add([4, 2, 6, 7]) ==> 2 
  */
const add = (lst) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
