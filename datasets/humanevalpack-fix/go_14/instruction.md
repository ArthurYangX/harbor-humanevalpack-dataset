# Context

```go
// Return list of all prefixes from shortest to longest of the input string
// >>> AllPrefixes('abc')
// ['a', 'ab', 'abc']
func AllPrefixes(str string) []string{
    prefixes := make([]string, 0, len(str))
	for i := 0; i < len(str)-1; i++ {
		prefixes = append(prefixes, str[:i+1])
	}
	return prefixes
}
```

# Instruction

Fix bugs in AllPrefixes.

# Prompt


// Return list of all prefixes from shortest to longest of the input string
// >>> AllPrefixes('abc')
// ['a', 'ab', 'abc']
func AllPrefixes(str string) []string{


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
