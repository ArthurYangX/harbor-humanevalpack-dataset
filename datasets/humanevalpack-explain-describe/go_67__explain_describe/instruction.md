# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
	"strconv"
	"strings"
)

// In this task, you will be given a string that represents a number of apples and oranges
// that are distributed in a basket of fruit this basket contains
// apples, oranges, and mango fruits. Given the string that represents the total number of
// the oranges and apples and an integer that represent the total number of the fruits
// in the basket return the number of the mango fruits in the basket.
// for examble:
// FruitDistribution("5 apples and 6 oranges", 19) ->19 - 5 - 6 = 8
// FruitDistribution("0 apples and 1 oranges",3) -> 3 - 0 - 1 = 2
// FruitDistribution("2 apples and 3 oranges", 100) -> 100 - 2 - 3 = 95
// FruitDistribution("100 apples and 1 oranges",120) -> 120 - 100 - 1 = 19
func FruitDistribution(s string,n int) int {
    split := strings.Split(s, " ")
	for _, i := range split {
		atoi, err := strconv.Atoi(i)
		if err != nil {
			continue
		}
		n = n - atoi
	}
	return n
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
	"strconv"
	"strings"
)

// In this task, you will be given a string that represents a number of apples and oranges
// that are distributed in a basket of fruit this basket contains
// apples, oranges, and mango fruits. Given the string that represents the total number of
// the oranges and apples and an integer that represent the total number of the fruits
// in the basket return the number of the mango fruits in the basket.
// for examble:
// FruitDistribution("5 apples and 6 oranges", 19) ->19 - 5 - 6 = 8
// FruitDistribution("0 apples and 1 oranges",3) -> 3 - 0 - 1 = 2
// FruitDistribution("2 apples and 3 oranges", 100) -> 100 - 2 - 3 = 95
// FruitDistribution("100 apples and 1 oranges",120) -> 120 - 100 - 1 = 19
func FruitDistribution(s string,n int) int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
