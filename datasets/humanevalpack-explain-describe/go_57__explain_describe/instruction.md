# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Return true is list elements are Monotonically increasing or decreasing.
// >>> Monotonic([1, 2, 4, 20])
// true
// >>> Monotonic([1, 20, 4, 10])
// false
// >>> Monotonic([4, 1, 0, -10])
// true
func Monotonic(l []int) bool {
    flag := true
	if len(l) > 1 {
		for i := 0; i < len(l)-1; i++ {
			if l[i] != l[i+1] {
				flag = l[i] > l[i+1]
				break
			}
		}
	} else {
		return false
	}
	for i := 0; i < len(l)-1; i++ {
		if flag != (l[i] >= l[i+1]) {
			return false
		}
	}
	return true
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


// Return true is list elements are Monotonically increasing or decreasing.
// >>> Monotonic([1, 2, 4, 20])
// true
// >>> Monotonic([1, 20, 4, 10])
// false
// >>> Monotonic([4, 1, 0, -10])
// true
func Monotonic(l []int) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
