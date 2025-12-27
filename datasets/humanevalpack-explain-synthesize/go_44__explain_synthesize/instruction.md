# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "strconv"
)

// Change numerical base of input number x to base.
// return string representation after the conversion.
// base numbers are less than 10.
// >>> ChangeBase(8, 3)
// '22'
// >>> ChangeBase(8, 2)
// '1000'
// >>> ChangeBase(7, 2)
// '111'
func ChangeBase(x int, base int) string {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
