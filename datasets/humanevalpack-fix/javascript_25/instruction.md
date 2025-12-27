# Context

```javascript
/* Return list of prime factors of given integer in the order from smallest to largest.
  Each of the factors should be listed number of times corresponding to how many times it appeares in factorization.
  Input number should be equal to the product of all factors
  >>> factorize(8)
  [2, 2, 2]
  >>> factorize(25)
  [5, 5]
  >>> factorize(70)
  [2, 5, 7]
  */
const factorize = (n) => {
  var fact = [], i = 0;
  while (i <= Math.sqrt(n) + 1) {
    if (n % i == 0) {
      fact.push(i);
      n = n / i;
    } else {
      i += 1;
    }
  }

  if (n > 1)
    fact.push(n);
  return fact;
}
```

# Instruction

Fix bugs in factorize.

# Prompt

/* Return list of prime factors of given integer in the order from smallest to largest.
  Each of the factors should be listed number of times corresponding to how many times it appeares in factorization.
  Input number should be equal to the product of all factors
  >>> factorize(8)
  [2, 2, 2]
  >>> factorize(25)
  [5, 5]
  >>> factorize(70)
  [2, 5, 7]
  */
const factorize = (n) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
