# Context

```javascript
/*The Brazilian factorial is defined as:
  brazilian_factorial(n) = n! * (n-1)! * (n-2)! * ... * 1!
  where n > 0

  For example:
  >>> specialFactorial(4)
  288

  The function will receive an integer as input and should return the special
  factorial of this integer.
  */
const specialFactorial = (n) => {
  let p = 1;
  let t = 1;
  while (n > 1) {
    let y = p;
    while (y > 0) {
      y--;
      n *= y;
      t *= n;
    }
    p++;
    p++;
    n--;
  }
  return t
}
```

# Instruction

Fix bugs in specialFactorial.

# Prompt

/*The Brazilian factorial is defined as:
  brazilian_factorial(n) = n! * (n-1)! * (n-2)! * ... * 1!
  where n > 0

  For example:
  >>> specialFactorial(4)
  288

  The function will receive an integer as input and should return the special
  factorial of this integer.
  */
const specialFactorial = (n) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
