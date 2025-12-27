# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Circular shift the digits of the integer x, shift the digits right by shift
// and return the result as a string.
// If shift > number of digits, return digits reversed.
// >>> CircularShift(12, 1)
// "21"
// >>> CircularShift(12, 2)
// "12"
func CircularShift(x int,shift int) string {
    s := strconv.Itoa(x)
	if shift > len(s) {
		runes := make([]rune, 0)
		for i := len(s)-1; i >= 0; i-- {
			runes = append(runes, rune(s[i]))
		}
		return string(runes)
	}else{
		return s[len(s)-shift:]+s[:len(s)-shift]
	}
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


// Circular shift the digits of the integer x, shift the digits right by shift
// and return the result as a string.
// If shift > number of digits, return digits reversed.
// >>> CircularShift(12, 1)
// "21"
// >>> CircularShift(12, 2)
// "12"
func CircularShift(x int,shift int) string {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
