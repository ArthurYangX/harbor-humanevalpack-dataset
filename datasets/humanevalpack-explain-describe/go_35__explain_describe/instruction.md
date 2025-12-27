# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Return maximum element in the list.
// >>> MaxElement([1, 2, 3])
// 3
// >>> MaxElement([5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10])
// 123
func MaxElement(l []int) int {
    max := l[0]
	for _, x := range l {
		if x > max {
			max = x
		}
	}
	return max
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


// Return maximum element in the list.
// >>> MaxElement([1, 2, 3])
// 3
// >>> MaxElement([5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10])
// 123
func MaxElement(l []int) int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
