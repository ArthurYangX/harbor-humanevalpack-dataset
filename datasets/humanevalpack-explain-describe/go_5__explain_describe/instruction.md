# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Insert a number 'delimeter' between every two consecutive elements of input list `numbers'
// >>> Intersperse([], 4)
// []
// >>> Intersperse([1, 2, 3], 4)
// [1, 4, 2, 4, 3]
func Intersperse(numbers []int, delimeter int) []int {
    result := make([]int, 0)
    if len(numbers) == 0 {
        return result
    }
    for i := 0; i < len(numbers)-1; i++ {
        n := numbers[i]
        result = append(result, n)
        result = append(result, delimeter)
    }
    result = append(result, numbers[len(numbers)-1])
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


// Insert a number 'delimeter' between every two consecutive elements of input list `numbers'
// >>> Intersperse([], 4)
// []
// >>> Intersperse([1, 2, 3], 4)
// [1, 4, 2, 4, 3]
func Intersperse(numbers []int, delimeter int) []int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
