# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Return the largest prime factor of n. Assume n > 1 and is not a prime.
// >>> LargestPrimeFactor(13195)
// 29
// >>> LargestPrimeFactor(2048)
// 2
func LargestPrimeFactor(n int) int {
    isPrime := func(n int) bool {
        for i := 2; i < int(math.Pow(float64(n), 0.5)+1); i++ {
            if n%i == 0 {
                return false
            }
        }
        return true
    }

    largest := 1
    for j := 2; j < n + 1; j++ {
		if n % j == 0 && isPrime(j) {
			if j > largest {
				largest = j
			}
		}
	}
    return largest
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


// Return the largest prime factor of n. Assume n > 1 and is not a prime.
// >>> LargestPrimeFactor(13195)
// 29
// >>> LargestPrimeFactor(2048)
// 2
func LargestPrimeFactor(n int) int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
