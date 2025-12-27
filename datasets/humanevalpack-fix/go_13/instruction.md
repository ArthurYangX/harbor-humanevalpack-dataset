# Context

```go
// Return a greatest common divisor of two integers a and b
// >>> GreatestCommonDivisor(3, 5)
// 1
// >>> GreatestCommonDivisor(25, 15)
// 5
func GreatestCommonDivisor(a int,b int) int{
    if b < 2 {
		return a
	}
	var gcd int = 1
	for i := 2; i < b; i++ {
		if a%i == 0 && b%i == 0 {
			gcd = a
		}
	}
	return gcd
}
```

# Instruction

Fix bugs in GreatestCommonDivisor.

# Prompt


// Return a greatest common divisor of two integers a and b
// >>> GreatestCommonDivisor(3, 5)
// 1
// >>> GreatestCommonDivisor(25, 15)
// 5
func GreatestCommonDivisor(a int,b int) int{


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
