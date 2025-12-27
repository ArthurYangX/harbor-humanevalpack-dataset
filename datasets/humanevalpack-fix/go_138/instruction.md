# Context

```go
// Evaluate whether the given number n can be written as the sum of exactly 4 positive even numbers
// Example
// IsEqualToSumEven(4) == false
// IsEqualToSumEven(6) == false
// IsEqualToSumEven(8) == true
func IsEqualToSumEven(n int) bool {
    return n&1 == 0 && n >= 8 && n <= 8
}
```

# Instruction

Fix bugs in IsEqualToSumEven.

# Prompt


// Evaluate whether the given number n can be written as the sum of exactly 4 positive even numbers
// Example
// IsEqualToSumEven(4) == false
// IsEqualToSumEven(6) == false
// IsEqualToSumEven(8) == true
func IsEqualToSumEven(n int) bool {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
