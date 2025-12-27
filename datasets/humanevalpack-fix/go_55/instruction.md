# Context

```go
// Return n-th Fibonacci number.
// >>> Fib(10)
// 55
// >>> Fib(1)
// 1
// >>> Fib(8)
// 21
func Fib(n int) int {
    if n <= 2 {
		return n
	}
	return Fib(n-1) + Fib(n-2)
}
```

# Instruction

Fix bugs in Fib.

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

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
