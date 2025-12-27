# Context

```javascript
/*Return true if a given number is prime, and false otherwise.
  >>> isPrime(6)
  false
  >>> isPrime(101)
  true
  >>> isPrime(11)
  true
  >>> isPrime(13441)
  true
  >>> isPrime(61)
  true
  >>> isPrime(4)
  false
  >>> isPrime(1)
  false
  */
const isPrime = (n) => {
  if (n < 1)
    return false;
  for (let k = 1; k < n - 1; k++)
    if (n % k == 0)
      return false;
  return true;
}
```

# Instruction

Fix bugs in isPrime.

# Prompt

/*Return true if a given number is prime, and false otherwise.
  >>> isPrime(6)
  false
  >>> isPrime(101)
  true
  >>> isPrime(11)
  true
  >>> isPrime(13441)
  true
  >>> isPrime(61)
  true
  >>> isPrime(4)
  false
  >>> isPrime(1)
  false
  */
const isPrime = (n) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
