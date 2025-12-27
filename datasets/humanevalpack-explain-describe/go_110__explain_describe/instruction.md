# Context

You are given a reference implementation (canonical solution) to explain.

```go
// In this problem, you will implement a function that takes two lists of numbers,
// and determines whether it is possible to perform an Exchange of elements
// between them to make lst1 a list of only even numbers.
// There is no limit on the number of Exchanged elements between lst1 and lst2.
// If it is possible to Exchange elements between the lst1 and lst2 to make
// all the elements of lst1 to be even, return "YES".
// Otherwise, return "NO".
// For example:
// Exchange([1, 2, 3, 4], [1, 2, 3, 4]) => "YES"
// Exchange([1, 2, 3, 4], [1, 5, 3, 4]) => "NO"
// It is assumed that the input lists will be non-empty.
func Exchange(lst1, lst2 []int) string {
    odd := 0
    even := 0
    for _, i := range lst1 {
        if i%2 == 1 {
            odd++
        }
    }
    for _, i := range lst2 {
        if i%2 == 0 {
            even++
        }
    }
    if even >= odd {
        return "YES"
    }
    return "NO"
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


// In this problem, you will implement a function that takes two lists of numbers,
// and determines whether it is possible to perform an Exchange of elements
// between them to make lst1 a list of only even numbers.
// There is no limit on the number of Exchanged elements between lst1 and lst2.
// If it is possible to Exchange elements between the lst1 and lst2 to make
// all the elements of lst1 to be even, return "YES".
// Otherwise, return "NO".
// For example:
// Exchange([1, 2, 3, 4], [1, 2, 3, 4]) => "YES"
// Exchange([1, 2, 3, 4], [1, 5, 3, 4]) => "NO"
// It is assumed that the input lists will be non-empty.
func Exchange(lst1, lst2 []int) string {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
