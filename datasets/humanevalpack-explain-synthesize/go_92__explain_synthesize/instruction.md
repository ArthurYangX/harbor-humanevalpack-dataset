# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt


// Create a function that takes 3 numbers.
// Returns true if one of the numbers is equal to the sum of the other two, and all numbers are integers.
// Returns false in any other cases.
// 
// Examples
// AnyInt(5, 2, 7) ➞ true
// 
// AnyInt(3, 2, 2) ➞ false
// 
// AnyInt(3, -2, 1) ➞ true
// 
// AnyInt(3.6, -2.2, 2) ➞ false
func AnyInt(x, y, z interface{}) bool {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
