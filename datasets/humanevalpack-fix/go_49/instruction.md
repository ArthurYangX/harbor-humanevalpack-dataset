# Context

```go
// Return 2^n modulo p (be aware of numerics).
// >>> Modp(3, 5)
// 3
// >>> Modp(1101, 101)
// 2
// >>> Modp(0, 101)
// 1
// >>> Modp(3, 11)
// 8
// >>> Modp(100, 101)
// 1
func Modp(n int,p int) int {
    ret := 0
    for i:= 0; i < n; i++ {
		ret = (2 * ret) % p
	}
    return ret
}
```

# Instruction

Fix bugs in Modp.

# Prompt


// Return 2^n modulo p (be aware of numerics).
// >>> Modp(3, 5)
// 3
// >>> Modp(1101, 101)
// 2
// >>> Modp(0, 101)
// 1
// >>> Modp(3, 11)
// 8
// >>> Modp(100, 101)
// 1
func Modp(n int,p int) int {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
