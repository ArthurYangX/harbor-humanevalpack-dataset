# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*The FibFib number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
  fibfib(0) == 0
  fibfib(1) == 0
  fibfib(2) == 1
  fibfib(n) == fibfib(n-1) + fibfib(n-2) + fibfib(n-3).
  Please write a function to efficiently compute the n-th element of the fibfib number sequence.
  >>> fibfib(1)
  0
  >>> fibfib(5)
  4
  >>> fibfib(8)
  24
  */
const fibfib = (n) => {
  if (n == 0 || n == 1)
    return 0;
  if (n == 2)
    return 1;
  return fibfib(n - 1) + fibfib(n - 2) + fibfib(n - 3);
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

/*The FibFib number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
  fibfib(0) == 0
  fibfib(1) == 0
  fibfib(2) == 1
  fibfib(n) == fibfib(n-1) + fibfib(n-2) + fibfib(n-3).
  Please write a function to efficiently compute the n-th element of the fibfib number sequence.
  >>> fibfib(1)
  0
  >>> fibfib(5)
  4
  >>> fibfib(8)
  24
  */
const fibfib = (n) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
