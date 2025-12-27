# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Find how many times a given substring can be found in the original string. Count overlaping cases.
// >>> HowManyTimes('', 'a')
// 0
// >>> HowManyTimes('aaa', 'a')
// 3
// >>> HowManyTimes('aaaa', 'aa')
// 3
func HowManyTimes(str string,substring string) int{
    times := 0
	for i := 0; i < (len(str) - len(substring) + 1); i++ {
		if str[i:i+len(substring)] == substring {
			times += 1
		}
	}
	return times
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


// Find how many times a given substring can be found in the original string. Count overlaping cases.
// >>> HowManyTimes('', 'a')
// 0
// >>> HowManyTimes('aaa', 'a')
// 3
// >>> HowManyTimes('aaaa', 'aa')
// 3
func HowManyTimes(str string,substring string) int{


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
