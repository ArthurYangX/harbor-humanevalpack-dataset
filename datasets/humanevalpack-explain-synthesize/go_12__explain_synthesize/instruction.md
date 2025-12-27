# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

// Out of list of strings, return the Longest one. Return the first one in case of multiple
// strings of the same length. Return nil in case the input list is empty.
// >>> Longest([])
// nil
// >>> Longest(['a', 'b', 'c'])
// 'a'
// >>> Longest(['a', 'bb', 'ccc'])
// 'ccc'
func Longest(strings []string) interface{}{


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
