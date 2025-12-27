# Context

```go
// SumToN is a function that sums numbers from 1 to n.
// >>> SumToN(30)
// 465
// >>> SumToN(100)
// 5050
// >>> SumToN(5)
// 15
// >>> SumToN(10)
// 55
// >>> SumToN(1)
// 1
func SumToN(n int) int {
    if n <= 0 {
		return 0
	} else {
		return 1 + SumToN(n - 1)
	}
}
```

# Instruction

Fix bugs in SumToN.

# Prompt


// SumToN is a function that sums numbers from 1 to n.
// >>> SumToN(30)
// 465
// >>> SumToN(100)
// 5050
// >>> SumToN(5)
// 15
// >>> SumToN(10)
// 55
// >>> SumToN(1)
// 1
func SumToN(n int) int {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
