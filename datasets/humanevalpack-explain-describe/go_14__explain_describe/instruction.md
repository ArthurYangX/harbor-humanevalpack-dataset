# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Return list of all prefixes from shortest to longest of the input string
// >>> AllPrefixes('abc')
// ['a', 'ab', 'abc']
func AllPrefixes(str string) []string{
    prefixes := make([]string, 0, len(str))
	for i := 0; i < len(str); i++ {
		prefixes = append(prefixes, str[:i+1])
	}
	return prefixes
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


// Return list of all prefixes from shortest to longest of the input string
// >>> AllPrefixes('abc')
// ['a', 'ab', 'abc']
func AllPrefixes(str string) []string{


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
