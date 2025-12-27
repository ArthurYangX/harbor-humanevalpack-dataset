# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
	"math"
)

// PrimeFib returns n-th number that is a Fibonacci number and it's also prime.
// >>> PrimeFib(1)
// 2
// >>> PrimeFib(2)
// 3
// >>> PrimeFib(3)
// 5
// >>> PrimeFib(4)
// 13
// >>> PrimeFib(5)
// 89
func PrimeFib(n int) int {
    isPrime := func(p int) bool {
		if p < 2 {
			return false
		}
		for i := 2; i < int(math.Min(math.Sqrt(float64(p))+1, float64(p-1))); i++ {
			if p%i == 0 {
				return false
			}
		}
		return true
	}
	f := []int{0, 1}
	for {
		f = append(f, f[len(f)-1]+f[len(f)-2])
		if isPrime(f[len(f)-1]) {
			n -= 1
		}
		if n == 0 {
			return f[len(f)-1]
		}
	}
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

import (
	"math"
)

// PrimeFib returns n-th number that is a Fibonacci number and it's also prime.
// >>> PrimeFib(1)
// 2
// >>> PrimeFib(2)
// 3
// >>> PrimeFib(3)
// 5
// >>> PrimeFib(4)
// 13
// >>> PrimeFib(5)
// 89
func PrimeFib(n int) int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
