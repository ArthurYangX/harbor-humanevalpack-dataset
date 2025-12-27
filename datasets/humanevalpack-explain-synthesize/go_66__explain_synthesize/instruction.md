# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt


// Task
// Write a function that takes a string as input and returns the sum of the upper characters only'
// ASCII codes.
// 
// Examples:
// Digitsum("") => 0
// Digitsum("abAB") => 131
// Digitsum("abcCd") => 67
// Digitsum("helloE") => 69
// Digitsum("woArBld") => 131
// Digitsum("aAaaaXa") => 153
func Digitsum(x string) int {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
