# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Evaluate whether the given number n can be written as the sum of exactly 4 positive even numbers
// Example
// IsEqualToSumEven(4) == false
// IsEqualToSumEven(6) == false
// IsEqualToSumEven(8) == true
func IsEqualToSumEven(n int) bool {
    return n&1 == 0 && n >= 8
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


// Evaluate whether the given number n can be written as the sum of exactly 4 positive even numbers
// Example
// IsEqualToSumEven(4) == false
// IsEqualToSumEven(6) == false
// IsEqualToSumEven(8) == true
func IsEqualToSumEven(n int) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
