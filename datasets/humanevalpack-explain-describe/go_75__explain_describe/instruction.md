# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Write a function that returns true if the given number is the multiplication of 3 prime numbers
// and false otherwise.
// Knowing that (a) is less then 100.
// Example:
// IsMultiplyPrime(30) == true
// 30 = 2 * 3 * 5
func IsMultiplyPrime(a int) bool {
    isPrime := func(n int) bool {
        for i := 2; i < int(math.Pow(float64(n), 0.5)+1); i++ {
            if n%i == 0 {
                return false
            }
        }
        return true
    }
    for i := 2; i < 101; i++ {
		if !isPrime(i) {
			continue
		}
		for j := 2; j < 101; j++ {
			if !isPrime(j) {
				continue
			}
			for k := 2; k < 101; k++ {
				if !isPrime(k) {
					continue
				}
				if i*j*k == a {
					return true
				}
			}
		}
	}
	return false
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


// Write a function that returns true if the given number is the multiplication of 3 prime numbers
// and false otherwise.
// Knowing that (a) is less then 100.
// Example:
// IsMultiplyPrime(30) == true
// 30 = 2 * 3 * 5
func IsMultiplyPrime(a int) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
