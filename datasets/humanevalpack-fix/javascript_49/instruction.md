# Context

```javascript
/*Return 2^n modulo p (be aware of numerics).
  >>> modp(3, 5)
  3
  >>> modp(1101, 101)
  2
  >>> modp(0, 101)
  1
  >>> modp(3, 11)
  8
  >>> modp(100, 101)
  1
  */
const modp = (n, p) => {
  var ret = 0;
  for (let i = 0; i < n; i++)
    ret = (2 * ret) % p;
  return ret;
}
```

# Instruction

Fix bugs in modp.

# Prompt

/*Return 2^n modulo p (be aware of numerics).
  >>> modp(3, 5)
  3
  >>> modp(1101, 101)
  2
  >>> modp(0, 101)
  1
  >>> modp(3, 11)
  8
  >>> modp(100, 101)
  1
  */
const modp = (n, p) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
