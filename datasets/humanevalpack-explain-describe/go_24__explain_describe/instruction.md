# Context

You are given a reference implementation (canonical solution) to explain.

```go
// For a given number n, find the largest number that divides n evenly, smaller than n
// >>> LargestDivisor(15)
// 5
func LargestDivisor(n int) int {
    for i := n - 1; i > 0; i-- {
		if n % i == 0 {
			return i
		}
	}
	return 0
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


// For a given number n, find the largest number that divides n evenly, smaller than n
// >>> LargestDivisor(15)
// 5
func LargestDivisor(n int) int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
