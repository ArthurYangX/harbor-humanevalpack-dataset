# Context

```go
// Create a function which returns the largest index of an element which
// is not greater than or equal to the element immediately preceding it. If
// no such element exists then return -1. The given array will not contain
// duplicate values.
// 
// Examples:
// CanArrange([1,2,4,3,5]) = 3
// CanArrange([1,2,3]) = -1
func CanArrange(arr []int) int {
    ind:=-1
    i:=1
    for i<len(arr) {
      if arr[i]<arr[i-1] {
        ind=i
      }
      i++
      ind--
    }
    return ind
}
```

# Instruction

Fix bugs in CanArrange.

# Prompt


// Create a function which returns the largest index of an element which
// is not greater than or equal to the element immediately preceding it. If
// no such element exists then return -1. The given array will not contain
// duplicate values.
// 
// Examples:
// CanArrange([1,2,4,3,5]) = 3
// CanArrange([1,2,3]) = -1
func CanArrange(arr []int) int {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
