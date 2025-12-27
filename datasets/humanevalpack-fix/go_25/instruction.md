# Context

```go
import (
    "math"
)

// Return list of prime factors of given integer in the order from smallest to largest.
// Each of the factors should be listed number of times corresponding to how many times it appeares in factorization.
// Input number should be equal to the product of all factors
// >>> Factorize(8)
// [2, 2, 2]
// >>> Factorize(25)
// [5, 5]
// >>> Factorize(70)
// [2, 5, 7]
func Factorize(n int) []int {
    fact := make([]int, 0)
	for i := 0; i <= int(math.Sqrt(float64(n))+1); {
		if n%i == 0 {
			fact = append(fact, i)
			n = n / i
		} else {
			i++
		}
	}
	if n > 1 {
		fact = append(fact, n)
	}
	return fact
}
```

# Instruction

Fix bugs in Factorize.

# Prompt

import (
    "math"
)

// Return list of prime factors of given integer in the order from smallest to largest.
// Each of the factors should be listed number of times corresponding to how many times it appeares in factorization.
// Input number should be equal to the product of all factors
// >>> Factorize(8)
// [2, 2, 2]
// >>> Factorize(25)
// [5, 5]
// >>> Factorize(70)
// [2, 5, 7]
func Factorize(n int) []int {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
