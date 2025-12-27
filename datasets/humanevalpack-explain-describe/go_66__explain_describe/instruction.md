# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Task
// Write a function that takes a string as input and returns the sum of the upper characters only'
// ASCII codes.
// 
// Examples:
// Digitsum("") => 0
// Digitsum("abAB") => 131
// Digitsum("abcCd") => 67
// Digitsum("helloE") => 69
// Digitsum("woArBld") => 131
// Digitsum("aAaaaXa") => 153
func Digitsum(x string) int {
    if len(x) == 0 {
		return 0
	}
	result := 0
	for _, i := range x {
		if 'A' <= i && i <= 'Z' {
			result += int(i)
		}
	}
	return result
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


// Task
// Write a function that takes a string as input and returns the sum of the upper characters only'
// ASCII codes.
// 
// Examples:
// Digitsum("") => 0
// Digitsum("abAB") => 131
// Digitsum("abcCd") => 67
// Digitsum("helloE") => 69
// Digitsum("woArBld") => 131
// Digitsum("aAaaaXa") => 153
func Digitsum(x string) int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
