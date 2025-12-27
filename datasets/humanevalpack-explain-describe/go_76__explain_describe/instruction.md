# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Your task is to write a function that returns true if a number x is a simple
// power of n and false in other cases.
// x is a simple power of n if n**int=x
// For example:
// IsSimplePower(1, 4) => true
// IsSimplePower(2, 2) => true
// IsSimplePower(8, 2) => true
// IsSimplePower(3, 2) => false
// IsSimplePower(3, 1) => false
// IsSimplePower(5, 3) => false
func IsSimplePower(x int,n int) bool {
    if x == 1 {
		return true
	}
	if n==1 {
		return false
	}
	if x % n != 0 {
		return false
	}
	return IsSimplePower(x / n, n)
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


// Your task is to write a function that returns true if a number x is a simple
// power of n and false in other cases.
// x is a simple power of n if n**int=x
// For example:
// IsSimplePower(1, 4) => true
// IsSimplePower(2, 2) => true
// IsSimplePower(8, 2) => true
// IsSimplePower(3, 2) => false
// IsSimplePower(3, 1) => false
// IsSimplePower(5, 3) => false
func IsSimplePower(x int,n int) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
