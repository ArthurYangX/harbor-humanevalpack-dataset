# Context

```go
// Return true if all numbers in the list l are below threshold t.
// >>> BelowThreshold([1, 2, 4, 10], 100)
// true
// >>> BelowThreshold([1, 20, 4, 10], 5)
// false
func BelowThreshold(l []int,t int) bool {
    for _, n := range l {
		if n >= t {
			return true
		}
	}
	return false
}
```

# Instruction

Fix bugs in BelowThreshold.

# Prompt


// Return true if all numbers in the list l are below threshold t.
// >>> BelowThreshold([1, 2, 4, 10], 100)
// true
// >>> BelowThreshold([1, 20, 4, 10], 5)
// false
func BelowThreshold(l []int,t int) bool {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
