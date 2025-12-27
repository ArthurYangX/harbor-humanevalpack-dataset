# Context

You are given a reference implementation (canonical solution) to explain.

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
		if s[:len(prefix)] == prefix {
			res = append(res, s)
		}
	}
	return res
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


// Filter an input list of strings only for ones that start with a given prefix.
// >>> FilterByPrefix([], 'a')
// []
// >>> FilterByPrefix(['abc', 'bcd', 'cde', 'array'], 'a')
// ['abc', 'array']
func FilterByPrefix(strings []string,prefix string) []string {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
