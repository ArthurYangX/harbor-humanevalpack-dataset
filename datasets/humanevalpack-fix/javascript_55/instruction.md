# Context

```javascript
/*Return n-th Fibonacci number.
  >>> fib(10)
  55
  >>> fib(1)
  1
  >>> fib(8)
  21
  */
const fib = (n) => {
  if (n == 0)
    return 0;
  if (n == 1)
    return 1;
  if (n == 2)
    return 2;
  return fib(n - 1) + fib(n - 2);
}
```

# Instruction

Fix bugs in fib.

# Prompt

/*Return n-th Fibonacci number.
  >>> fib(10)
  55
  >>> fib(1)
  1
  >>> fib(8)
  21
  */
const fib = (n) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
