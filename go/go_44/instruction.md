# Problem Description

Write a Go function `func ChangeBase(x int, base int) string` to solve the following problem:
Change numerical base of input number x to base.
return string representation after the conversion.
base numbers are less than 10.
>>> ChangeBase(8, 3)
'22'
>>> ChangeBase(8, 2)
'1000'
>>> ChangeBase(7, 2)
'111'

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

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
