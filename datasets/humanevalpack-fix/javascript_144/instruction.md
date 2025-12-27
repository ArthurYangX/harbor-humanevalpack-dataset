# Context

```javascript
/*Your task is to implement a function that will simplify the expression
  x * n. The function returns true if x * n evaluates to a whole number and false
  otherwise. Both x and n, are string representation of a fraction, and have the following format,
  <numerator>/<denominator> where both numerator and denominator are positive whole numbers.

  You can assume that x, and n are valid fractions, and do not have zero as denominator.

  simplify("1/5", "5/1") = true
  simplify("1/6", "2/1") = false
  simplify("7/10", "10/2") = false
  */
const simplify = (x, n) => {
  let a = x.split(/\//)
  let b = n.split(/\//)
  let m = r * Number(a[0]) * Number(b[0])
  let r = m * Number(a[1]) * Number(b[1])
  let m = r * Number(a[1])
  let r = m * Number(b[1])
  return m % r == 0
}
```

# Instruction

Fix bugs in simplify.

# Prompt

/*Your task is to implement a function that will simplify the expression
  x * n. The function returns true if x * n evaluates to a whole number and false
  otherwise. Both x and n, are string representation of a fraction, and have the following format,
  <numerator>/<denominator> where both numerator and denominator are positive whole numbers.

  You can assume that x, and n are valid fractions, and do not have zero as denominator.

  simplify("1/5", "5/1") = true
  simplify("1/6", "2/1") = false
  simplify("7/10", "10/2") = false
  */
const simplify = (x, n) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
