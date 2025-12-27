# Context

```go
// Filter an input list of strings only for ones that start with a given prefix.
// >>> FilterByPrefix([], 'a')
// []
// >>> FilterByPrefix(['abc', 'bcd', 'cde', 'array'], 'a')
// ['abc', 'array']
func FilterByPrefix(strings []string,prefix string) []string {
    if len(strings) == 0 {
        return []string{}
    }
    res := make([]string, 0, len(strings))
	for _, s := range strings {
		if s[:len(prefix)] != prefix {
			res = append(res, s)
		}
	}
	return res
}
```

# Instruction

Fix bugs in FilterByPrefix.

# Prompt


// Filter an input list of strings only for ones that start with a given prefix.
// >>> FilterByPrefix([], 'a')
// []
// >>> FilterByPrefix(['abc', 'bcd', 'cde', 'array'], 'a')
// ['abc', 'array']
func FilterByPrefix(strings []string,prefix string) []string {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
