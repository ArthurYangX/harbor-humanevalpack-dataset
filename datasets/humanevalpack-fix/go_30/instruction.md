# Context

```go
// Return only positive numbers in the list.
// >>> GetPositive([-1, 2, -4, 5, 6])
// [2, 5, 6]
// >>> GetPositive([5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10])
// [5, 3, 2, 3, 9, 123, 1]
func GetPositive(l []int) []int {
    res := make([]int, 0)
    for _, x := range l {
        if x < 0 {
            res = append(res, x)
        }
    }
    return res
}
```

# Instruction

Fix bugs in GetPositive.

# Prompt


// Return only positive numbers in the list.
// >>> GetPositive([-1, 2, -4, 5, 6])
// [2, 5, 6]
// >>> GetPositive([5, 3, -5, 2, -3, 3, 9, 0, 123, 1, -10])
// [5, 3, 2, 3, 9, 123, 1]
func GetPositive(l []int) []int {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
