# Context

```go
// Concatenate list of strings into a single string
// >>> Concatenate([])
// ''
// >>> Concatenate(['a', 'b', 'c'])
// 'abc'
func Concatenate(strings []string) string {
    if len(strings) == 0 {
		return ""
	}
	return Concatenate(strings[1:])
}
```

# Instruction

Fix bugs in Concatenate.

# Prompt


// Concatenate list of strings into a single string
// >>> Concatenate([])
// ''
// >>> Concatenate(['a', 'b', 'c'])
// 'abc'
func Concatenate(strings []string) string {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
