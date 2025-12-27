# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Given an array arr of integers, find the minimum number of elements that
// need to be changed to make the array palindromic. A palindromic array is an array that
// is read the same backwards and forwards. In one change, you can change one element to any other element.
// 
// For example:
// SmallestChange([1,2,3,5,4,7,9,6]) == 4
// SmallestChange([1, 2, 3, 4, 3, 2, 2]) == 1
// SmallestChange([1, 2, 3, 2, 1]) == 0
func SmallestChange(arr []int) int {
    count := 0
	for i := 0; i < len(arr) - 1; i++ {
        a := arr[len(arr) - i - 1]
		if arr[i] != a {
			arr[i] = a
            count++
		}
	}
	return count
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


// Given an array arr of integers, find the minimum number of elements that
// need to be changed to make the array palindromic. A palindromic array is an array that
// is read the same backwards and forwards. In one change, you can change one element to any other element.
// 
// For example:
// SmallestChange([1,2,3,5,4,7,9,6]) == 4
// SmallestChange([1, 2, 3, 4, 3, 2, 2]) == 1
// SmallestChange([1, 2, 3, 2, 1]) == 0
func SmallestChange(arr []int) int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
