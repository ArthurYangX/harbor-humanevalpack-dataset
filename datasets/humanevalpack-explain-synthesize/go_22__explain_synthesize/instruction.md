# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

// Filter given list of any values only for integers
// >>> FilterIntegers(['a', 3.14, 5])
// [5]
// >>> FilterIntegers([1, 2, 3, 'abc', {}, []])
// [1, 2, 3]
func FilterIntegers(values []interface{}) []int {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
