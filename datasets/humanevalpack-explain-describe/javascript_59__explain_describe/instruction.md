# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Return the largest prime factor of n. Assume n > 1 and is not a prime.
  >>> largestPrimeFactor(13195)
  29
  >>> largestPrimeFactor(2048)
  2
  */
const largestPrimeFactor = (n) => {
  var isPrime = function (k) {
    if (k < 2)
      return false;
    for (let i = 2; i < k - 1; i++)
      if (k % i == 0)
        return false;
    return true;
  }

  var largest = 1;
  for (let j = 2; j < n + 1; j++)
    if (n % j == 0 && isPrime(j))
      largest = Math.max(largest, j);
  return largest;
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

/*Return the largest prime factor of n. Assume n > 1 and is not a prime.
  >>> largestPrimeFactor(13195)
  29
  >>> largestPrimeFactor(2048)
  2
  */
const largestPrimeFactor = (n) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
