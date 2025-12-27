# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
	"fmt"
)

// You will be given a number in decimal form and your task is to convert it to
// binary format. The function should return a string, with each character representing a binary
// number. Each character in the string will be '0' or '1'.
// 
// There will be an extra couple of characters 'db' at the beginning and at the end of the string.
// The extra characters are there to help with the format.
// 
// Examples:
// DecimalToBinary(15)   # returns "db1111db"
// DecimalToBinary(32)   # returns "db100000db"
func DecimalToBinary(decimal int) string {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
