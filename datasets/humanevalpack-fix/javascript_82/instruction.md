# Context

```javascript
/*Write a function that takes a string and returns true if the string
  length is a prime number or false otherwise
  Examples
  primeLength('Hello') == true
  primeLength('abcdcba') == true
  primeLength('kittens') == true
  primeLength('orange') == false
  */
const primeLength = (string) => {
  let len = string.length
  if (len == 1 || len == 0) { return false }
  for (let i = 3; i * i <= len; i++) {
    if (len % i == 0) { return false }
  }
  return true
}
```

# Instruction

Fix bugs in primeLength.

# Prompt

/*Write a function that takes a string and returns true if the string
  length is a prime number or false otherwise
  Examples
  primeLength('Hello') == true
  primeLength('abcdcba') == true
  primeLength('kittens') == true
  primeLength('orange') == false
  */
const primeLength = (string) => {


# Instructions

Implement your solution in `solution/solution.js`.
Ensure your submission is self-contained and compiles/runs correctly.

```
