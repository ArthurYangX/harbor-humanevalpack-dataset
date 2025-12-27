# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Return only positive numbers in the list.
// >>> GetPositive([-1, 2, -4, 5, 6])
// [2, 5, 6]
// >>> GetPositive([5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10])
// [5, 3, 2, 3, 9, 123, 1]
func GetPositive(l []int) []int {
    res := make([]int, 0)
    for _, x := range l {
        if x > 0 {
            res = append(res, x)
        }
    }
    return res
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


// Return only positive numbers in the list.
// >>> GetPositive([-1, 2, -4, 5, 6])
// [2, 5, 6]
// >>> GetPositive([5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10])
// [5, 3, 2, 3, 9, 123, 1]
func GetPositive(l []int) []int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
