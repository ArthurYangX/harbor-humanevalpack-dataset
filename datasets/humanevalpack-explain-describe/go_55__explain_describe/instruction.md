# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Return n-th Fibonacci number.
// >>> Fib(10)
// 55
// >>> Fib(1)
// 1
// >>> Fib(8)
// 21
func Fib(n int) int {
    if n <= 1 {
		return n
	}
	return Fib(n-1) + Fib(n-2)
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


// Return n-th Fibonacci number.
// >>> Fib(10)
// 55
// >>> Fib(1)
// 1
// >>> Fib(8)
// 21
func Fib(n int) int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
