# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Return a greatest common divisor of two integers a and b
// >>> GreatestCommonDivisor(3, 5)
// 1
// >>> GreatestCommonDivisor(25, 15)
// 5
func GreatestCommonDivisor(a int,b int) int{
    if b < 2 {
		return b
	}
	var gcd int = 1
	for i := 2; i < b; i++ {
		if a%i == 0 && b%i == 0 {
			gcd = i
		}
	}
	return gcd
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


// Return a greatest common divisor of two integers a and b
// >>> GreatestCommonDivisor(3, 5)
// 1
// >>> GreatestCommonDivisor(25, 15)
// 5
func GreatestCommonDivisor(a int,b int) int{


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
