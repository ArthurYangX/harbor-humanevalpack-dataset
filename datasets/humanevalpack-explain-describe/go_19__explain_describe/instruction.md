# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "sort"
    "strings"
)

// Input is a space-delimited string of numberals from 'zero' to 'nine'.
// Valid choices are 'zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight' and 'nine'.
// Return the string with numbers sorted from smallest to largest
// >>> SortNumbers('three one five')
// 'one three five'
func SortNumbers(numbers string) string{
    valueMap := map[string]int{
		"zero":  0,
		"one":   1,
		"two":   2,
		"three": 3,
		"four":  4,
		"five":  5,
		"six":   6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
	}
	stringMap := make(map[int]string)
	for s, i := range valueMap {
		stringMap[i] = s
	}
	split := strings.Split(numbers, " ")
	temp := make([]int, 0)
	for _, s := range split {
		if i, ok := valueMap[s]; ok {
			temp = append(temp, i)
		}
	}
	sort.Ints(temp)
	result := make([]string, 0)
	for _, i := range temp {
		result = append(result, stringMap[i])
	}
	return strings.Join(result, " ")
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
    "sort"
    "strings"
)

// Input is a space-delimited string of numberals from 'zero' to 'nine'.
// Valid choices are 'zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight' and 'nine'.
// Return the string with numbers sorted from smallest to largest
// >>> SortNumbers('three one five')
// 'one three five'
func SortNumbers(numbers string) string{


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
